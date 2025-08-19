use std::error::Error;
use std::process::Command;

use serde::Serialize;
use tempfile::NamedTempFile;

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