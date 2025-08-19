use std::error::Error;
use std::process::Command;

use reqwest::blocking::Client;

use code2nix::*;

const QUERY_FLAGS: u32 = 0x80 ^ 0x200; // IncludeAssetsURI + IncludeLatestVersionOnly
const QUERY_FILTER_TYPE: u32 = 7; // ExtensionName

fn main() -> Result<(), Box<dyn Error>> {
  let client = Client::new();
  let out = Command::new("code")
    .args([ "--list-extensions" ])
    .output()?;

  let mut query = api::QueryBody {
    filters: Vec::new(),
    flags: QUERY_FLAGS,
  };

  for line in String::from_utf8(out.stdout)?.lines() {
    query.filters.push(api::QueryFilter {
      criteria: vec![api::QueryCriteria {
        filter_type: QUERY_FILTER_TYPE,
        value: line.to_string(),
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

  println!("{}", nix::to_string(&exts)?);

  return Ok(());
}
