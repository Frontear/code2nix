use std::fmt::Display;

use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct Extension {
  pub name: String,
  pub publisher: String,

  #[serde(default)]
  pub arch: String,
  #[serde(default, skip_deserializing)]
  pub hash: String,
  #[serde(default, skip_deserializing)]
  pub version: String,
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      return f.write_fmt(format_args!("{}.{}", self.publisher, self.name));
    }
}