use std::path::PathBuf;

use clap::{
  Args,
  Parser,
  Subcommand,
};

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct CliArgs {
  #[command(subcommand)]
  pub command: ExtSource,
}

#[derive(Debug, Subcommand)]
#[command(disable_help_subcommand = true)]
pub enum ExtSource {
  /// Process extensions from your VSCode installation
  Code,

  /// Process extensions from a pre-generated Nix file
  File(FileArgs),
}

#[derive(Debug, Args)]
pub struct FileArgs {
  /// Path to the Nix file
  pub file: PathBuf,
}
