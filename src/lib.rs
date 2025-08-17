mod args;
mod extension;

use std::process::Command;

pub use args::{
  CliArgs,
  ExtSource,
};

pub use extension::Extension;

pub fn run_cmd(binary: &str, args: Vec<&str>) -> String {
  let stdout = Command::new(binary)
    .args(&args)
    .output()
    .expect(format!("Failed to run '{binary}' with args: '{:?}'", &args).as_str());

  return String::from_utf8(stdout.stdout).unwrap();
}
