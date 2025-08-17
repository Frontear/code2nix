use serde::Deserialize;
use serde::Serialize;

fn construct_url(extension: &Extension, version: &String) -> String {
  let arch_flag = match &extension.arch {
    None => "".to_owned(),
    Some(arch) => format!("?targetPlatform={}", arch),
  };

  return format!("https://{0}.gallery.vsassets.io/_apis/public/gallery/publisher/{0}/extension/{1}/{2}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage{3}", extension.publisher, extension.name, version, arch_flag);
}

#[derive(Deserialize, Serialize)]
pub struct Extension {
  pub publisher: String,
  pub name: String,
  pub version: Option<String>,
  pub arch: Option<String>,
  pub hash: Option<String>,
}

impl Extension {
  pub fn url(&self) -> String {
    return match &self.version {
      None => construct_url(&self, &"latest".to_owned()),
      Some(version) => construct_url(&self, version),
    };
  }
}
