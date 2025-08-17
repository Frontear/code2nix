use std::error::Error;
use std::io::Cursor;
use std::io::Read;
use std::path::PathBuf;

use clap::Parser;
use reqwest::blocking::Client;
use serde_json::Value;
use sha256;
use tempfile::NamedTempFile;
use zip::ZipArchive;

use code2nix::*;

fn from_code() -> Vec<Extension> {
  let mut exts = Vec::new();

  let out = run_cmd("code", vec![ "--list-extensions", "--show-versions" ]);
  let lines = out.lines();

  for line in lines {
    let (publisher, name_version) = line.split_once('.').unwrap();
    let (name, version) = name_version.split_once('@').unwrap();

    exts.push(Extension {
      publisher: publisher.to_owned(),
      name: name.to_owned(),
      version: Some(version.to_owned()),
      arch: None,
      hash: None,
    });
  }

  return exts;
}

fn from_file(file: &PathBuf) -> Vec<Extension> {
  let mut exts: Vec<Extension>;

  let out = run_cmd("nix", vec![ "eval", "--json", "--file", file.to_str().unwrap() ]);
  exts = serde_json::from_str(out.as_str()).unwrap();

  for ext in exts.iter_mut() {
    // forcefully resolve the latest version
    ext.version = None;
  }

  return exts;
}

fn download_ext(client: &Client, ext: &mut Extension) -> Result<(), Box<dyn Error>> {
  let resp = client
    .get(ext.url())
    .send()
    .expect(format!("Failed to download extension ({}.{}).", ext.publisher, ext.name).as_str())
    .bytes()?;

  if ext.version == None || ext.arch == None {
    let mut archive = ZipArchive::new(Cursor::new(resp.as_ref()))?;
    let json: Value = serde_json::from_reader(archive.by_name("extension/package.json")?)?;

    ext.version = Some(json.as_object().unwrap()["version"].as_str().unwrap().to_owned());

    let mut content = String::new();
    archive.by_name("extension.vsixmanifest")?.read_to_string(&mut content)?;

    if content.contains("TargetPlatform") {
      // TODO: this is hardcoded to 'linux-x64' since that's all I use
      ext.arch = Some("linux-x64".to_owned());
    }
    else {
      ext.arch = Some("".to_owned());
    }

    // force a re-download with the new version and architecture fields
    // to obtain the correct extension zip for hashing.
    //
    // See https://github.com/Frontear/code2nix/issues/2 for details.
    return download_ext(client, ext);
  }

  let digest = sha256::digest(resp.as_ref());
  let out = run_cmd("nix", vec![ "hash", "to-sri", "--type", "sha256", &digest ]);

  ext.hash = Some(out);

  return Ok(());
}

fn main() -> Result<(), Box<dyn Error>> {
  let args = CliArgs::parse();
  let mut exts = match &args.command {
    ExtSource::Code => from_code(),
    ExtSource::File(args) => from_file(&args.file),
  };

  let client = Client::new();
  for ext in exts.iter_mut() {
    download_ext(&client, ext)?;
  }

  let file = NamedTempFile::new()?;
  serde_json::to_writer(&file, &exts)?;
  let out = run_cmd("nix", vec![ "eval", "--impure", "--expr", format!("builtins.fromJSON (builtins.readFile {})", &file.path().to_str().unwrap()).as_str() ]);

  println!("{}", out);

  return Ok(());
}
