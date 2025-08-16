use std::error::Error;

use reqwest::blocking::Client;
use sha256;
use tempfile::NamedTempFile;

use code2nix::*;

fn from_code() -> Vec<Extension> {
  let mut exts = Vec::new();

  let out = run_cmd("code", vec![ "--list-extensions", "--show-versions" ]);
  let lines = out.lines();

  for line in lines {
    exts.push(Extension::from_code(line));
  }

  return exts;
}

fn download_ext(client: &Client, ext: &mut Extension) -> Result<(), Box<dyn Error>> {
  let resp = client
    .get(ext.url())
    .send()
    .expect(format!("Failed to download extension ({}.{}@{}).", ext.publisher, ext.name, ext.version).as_str());

  let digest = sha256::digest(resp.bytes()?.as_ref());
  let out = run_cmd("nix", vec![ "hash", "to-sri", "--type", "sha256", &digest ]);

  ext.hash = Some(out);

  return Ok(());
}

fn main() -> Result<(), Box<dyn Error>> {
  let client = Client::new();
  let mut exts = from_code();

  for mut ext in exts.iter_mut() {
    download_ext(&client, &mut ext)?;
  }

  let file = NamedTempFile::new()?;
  serde_json::to_writer(&file, &exts)?;
  let out = run_cmd("nix", vec![ "eval", "--impure", "--expr", format!("builtins.fromJSON (builtins.readFile {})", file.path().to_str().unwrap()).as_str() ]);

  println!("{}", out);

  return Ok(());
}
