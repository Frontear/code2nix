use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

use clap::Parser;
use reqwest::blocking::Client;

use code2nix::*;

const QUERY_FLAGS: u32 = 0x80 ^ 0x200; // IncludeAssetsURI + IncludeLatestVersionOnly
const QUERY_FILTER_TYPE: u32 = 7; // ExtensionName

fn parse_code() -> Result<Vec<String>, Box<dyn Error>> {
  let out = Command::new("code")
    .args([ "--list-extensions" ])
    .output()?;

  return Ok(String::from_utf8(out.stdout)?.lines().map(|s| s.into()).collect());
}

fn parse_file(path: impl AsRef<Path>) -> Result<Vec<String>, Box<dyn Error>> {
  let mut exts = Vec::new();

  for ext in nix::from_path::<Vec<models::Extension>>(path)?.into_iter() {
    exts.push(format!("{}.{}", ext.publisher, ext.name));
  }

  return Ok(exts);
}

fn main() -> Result<(), Box<dyn Error>> {
  let args = cli::Args::parse();
  let client = Client::new();
  let exts = match args.file {
    Some(path) => parse_file(&path),
    None => parse_code(),
  }?;

  let mut query = api::QueryBody {
    filters: Vec::new(),
    flags: QUERY_FLAGS,
  };

  for line in exts.into_iter() {
    query.filters.push(api::QueryFilter {
      criteria: vec![api::QueryCriteria {
        filter_type: QUERY_FILTER_TYPE,
        value: line,
      }],
    });
  }

  let resp = client
    .post("https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery?api-version=7.2-preview.1")
    .json(&query)
    .send()?
    .bytes()?;
  let resp: api::ResponseBody = serde_json::from_slice(resp.as_ref())?;

  let mut exts = Vec::new();
  for mut res in resp.results.into_iter() {
    let ext = res.extensions.pop().unwrap();

    let ver_info = ext.versions.into_iter().find(|x| {
      // NOTE: hard-coded 'linux-x64' because I don't use anything else.
      return x.target_platform.is_empty() || x.target_platform == "linux-x64";
    }).unwrap();

    let resp = client
      .get(format!("{}/Microsoft.VisualStudio.Services.VSIXPackage", ver_info.asset_uri))
      .send()?
      .bytes()?;

    exts.push(models::Extension {
      name: ext.extension_name,
      publisher: ext.publisher.publisher_name,
      version: ver_info.version,

      arch: ver_info.target_platform,
      hash: nix::hash(resp.as_ref()),
    });
  }

  let expr = nix::to_string(&exts)?;
  match args.out {
    Some(path) => fs::write(&path, expr)?,
    None => println!("{}", expr),
  }

  return Ok(());
}
