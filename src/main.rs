use std::error::Error;
use std::process::Command;

use reqwest::blocking::Client;
use serde::{ Deserialize, Serialize };
use tempfile::NamedTempFile;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawGalleryExtensionPublisher {
  publisher_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawGalleryExtensionVersion {
  asset_uri: String,
  #[serde(default)]
  target_platform: String,
  version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawGalleryExtension {
  extension_name: String,
  publisher: RawGalleryExtensionPublisher,
  versions: Vec<RawGalleryExtensionVersion>,
}

#[derive(Debug, Deserialize)]
struct RawGalleryQueryResultType {
  extensions: Vec<RawGalleryExtension>,
}

#[derive(Debug, Deserialize)]
struct RawGalleryQueryResult {
  results: Vec<RawGalleryQueryResultType>,
}

#[derive(Debug, Serialize)]
struct Extension {
  arch: String,
  hash: String,
  name: String,
  publisher: String,
  version: String,
}

fn main() -> Result<(), Box<dyn Error>> {
  let out = Command::new("code")
    .args([ "--list-extensions", "--show-versions" ])
    .output()?;

  let mut exts = Vec::new();
  for line in str::from_utf8(&out.stdout)?.lines() {
    let (publisher, name_version) = line.split_once('.').unwrap();
    let (name, version) = name_version.split_once('@').unwrap();

    exts.push(Extension {
      arch: "".into(),
      hash: "".into(),
      name: name.into(),
      publisher: publisher.into(),
      version: version.into(),
    });
  }

  let client = Client::new();
  let mut body = String::from(r#"{
    filters: [{
      criteria: [@criteria@]
    }],

    flags: @flags@
  }"#);

  let mut criteria = String::new();
  for ext in exts.iter() {
    criteria.push_str(format!("{{ filterType: 7, value: \"{}.{}\" }}", &ext.publisher, &ext.name).as_str());
    criteria.push(',');
  }
  criteria.pop(); // remove trailing comma


  let mut flags = 0;
  flags ^= 0x80; // IncludeAssetsURI
  flags ^= 0x200; // IncludeLatestVersionOnly

  body = body
    .replace("@criteria@", &criteria)
    .replace("@flags@", &flags.to_string());

  let resp = client
    .post("https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery?api-version=7.2-preview.1")
    .header("Content-Type", "application/json")
    .header("Accept", "application/json")
    .body(body)
    .send()?
    .bytes()?;
  let mut json: RawGalleryQueryResult = serde_json::from_slice(resp.as_ref())?;

  exts.clear();
  for ext in json.results.pop().unwrap().extensions.iter() {
    let name = ext.extension_name.clone();
    let publisher = ext.publisher.publisher_name.clone();
    let valid_version = ext.versions.iter().find(|&v|
      v.target_platform.is_empty() || v.target_platform == "linux-x64"
    ).unwrap();
    let version = valid_version.version.clone();
    let arch = valid_version.target_platform.clone();

    let resp = client
      .get(valid_version.asset_uri.clone() + "/Microsoft.VisualStudio.Services.VSIXPackage".into())
      .send()?
      .bytes()?;

    let digest = sha256::digest(resp.as_ref());
    let out = Command::new("nix")
      .args([ "hash", "to-sri", "--type", "sha256", &digest ])
      .output()?;

    let hash = String::from_utf8(out.stdout)?;

    exts.push(Extension {
      arch,
      hash,
      name,
      publisher,
      version,
    });
  }

  let file = NamedTempFile::new()?;
  serde_json::to_writer(&file, &exts)?;
  let out = Command::new("nix")
    .args([ "eval", "--impure", "--expr", format!("builtins.fromJSON (builtins.readFile {})", &file.path().to_str().unwrap()).as_str() ])
    .output()?;

  println!("{}", str::from_utf8(&out.stdout)?);

  return Ok(());
}
