use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

use clap::Parser;
use reqwest::blocking::Client;

use code2nix::*;

fn parse_code() -> Result<Vec<models::Extension>, Box<dyn Error>> {
  let mut exts = Vec::new();
  let out = Command::new("code")
    .arg("--list-extensions")
    .output()?;

  for line in String::from_utf8(out.stdout)?.lines() {
    let (publisher, name) = line.split_once('.').unwrap();

    exts.push(models::Extension {
      name: name.into(),
      publisher: publisher.into(),
      version: "".into(),

      arch: "".into(),
      hash: "".into(),
    });
  }

  return Ok(exts);
}

fn parse_file(path: impl AsRef<Path>) -> Result<Vec<models::Extension>, Box<dyn Error>> {
  let mut exts = Vec::new();

  for ext in nix::from_path::<Vec<models::Extension>>(path)?.into_iter() {
    exts.push(ext);
  }

  return Ok(exts);
}

fn main() -> Result<(), Box<dyn Error>> {
  let args = cli::Args::parse();
  let client = Client::new();
  let mut exts = match args.file {
    Some(path) => parse_file(&path),
    None => parse_code(),
  }?;

  let mut query = api::QueryBody {
    filters: Vec::new(),
    flags: vec![api::QueryFlags::IncludeAssetUri, api::QueryFlags::IncludeLatestVersionOnly],
  };

  for ext in exts.iter() {
    query.filters.push(api::QueryFilter {
      criteria: vec![api::QueryCriteria {
        filter_type: api::QueryFilterType::ExtensionName,
        value: format!("{}", ext),
      }],
    });
  }

  let resp = client
    .post(api::ENDPOINT)
    .json(&query)
    .send()?
    .bytes()?;
  let resp: api::ResponseBody = serde_json::from_slice(resp.as_ref())?;

  let mut exts_iter = exts.iter_mut();
  let mut resp_iter = resp.results.into_iter();

  while let Some(ext) = exts_iter.next() {
    let mut resp_extensions = resp_iter.next().unwrap().extensions;
    if resp_extensions.len() != 1 {
      eprintln!("Failed to query extension {}", ext);

      continue;
    }

    let resp_ext = resp_extensions.pop().unwrap();
    let ver_info = resp_ext.versions.into_iter().find(|v| {
      return v.target_platform.is_empty() || v.target_platform == "linux-x64";
    }).unwrap();

    let resp = client
      .get(format!("{}/Microsoft.VisualStudio.Services.VSIXPackage", ver_info.asset_uri))
      .send()?
      .bytes()?;

    ext.arch = ver_info.target_platform;
    ext.hash = nix::hash(resp.as_ref());
    ext.version = ver_info.version;
  }

  let expr = nix::to_string(&exts)?;
  match args.out {
    Some(path) => fs::write(&path, expr)?,
    None => println!("{}", expr),
  }

  return Ok(());
}
