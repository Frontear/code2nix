use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Args {
  /// If specified, read extensions from here instead of VSCode
  #[arg(short, long)]
  pub file: Option<PathBuf>,

  /// If specified, write expression to file, else stdout
  #[arg(short, long)]
  pub out: Option<PathBuf>,
}