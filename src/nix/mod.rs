use std::error::Error;
use std::path::Path;
use std::process::Command;

use serde::{ de::DeserializeOwned, Serialize };
use tempfile::NamedTempFile;

pub fn from_path<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, Box<dyn Error>> {
  let out = Command::new("nix")
    .args([ "eval", "--json", "--file", path.as_ref().to_str().unwrap() ])
    .output()?;

  return Ok(serde_json::from_slice(&out.stdout)?);
}

pub fn to_string<T: Serialize>(value: &T) -> Result<String, Box<dyn Error>> {
  let file = NamedTempFile::new()?;
  serde_json::to_writer(&file, value)?;

  let path = &file.path().to_str().unwrap();
  let out = Command::new("nix")
    .args([
      "eval", "--impure", "--expr",
      format!("builtins.fromJSON (builtins.readFile {})", path).as_str()
    ]).output()?;

  return Ok(String::from_utf8(out.stdout)?);
}

pub fn hash(content: &[u8]) -> String {
  let digest = sha256::digest(content);
  let out = Command::new("nix")
    .args([ "hash", "to-sri", "--type", "sha256", &digest ])
    .output()
    .unwrap();

  return String::from_utf8(out.stdout).unwrap();
}
