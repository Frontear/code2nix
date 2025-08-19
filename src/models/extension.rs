use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct Extension {
  pub name: String,
  pub publisher: String,
  pub version: String,

  #[serde(default)]
  pub arch: String,
  pub hash: String,
}