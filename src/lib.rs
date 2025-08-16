use std::process::Command;

use serde::{
  Deserialize,
  Serialize
};

pub fn run_cmd(binary: &str, args: Vec<&str>) -> String {
  let stdout = Command::new(binary)
    .args(&args)
    .output()
    .expect(format!("Failed to run '{binary}' with args: '{:?}'", &args).as_str());

  return String::from_utf8(stdout.stdout).unwrap();
}

#[derive(Deserialize, Serialize)]
pub struct Extension {
  pub publisher: String,
  pub name: String,
  pub version: String,
  pub hash: Option<String>,
}

impl Extension {
  pub fn from_code(line: &str) -> Self {
    let (publisher, name_version) = line.split_once('.').unwrap();
    let (name, version) = name_version.split_once('@').unwrap();

    return Extension {
      publisher: publisher.to_owned(),
      name: name.to_owned(),
      version: version.to_owned(),
      hash: None,
    };
  }

  pub fn url(&self) -> String {
    return format!("https://{0}.gallery.vsassets.io/_apis/public/gallery/publisher/{0}/extension/{1}/{2}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage", self.publisher, self.name, self.version);
  }
}
