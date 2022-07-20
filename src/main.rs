use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use home::home_dir;

use rmbuild::visit;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// starting path for rmbuild, default is your home directory
    #[clap(value_parser)]
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let root_path = args.path.unwrap_or(home());
    visit(root_path)?;
    Ok(())
}

/// return the home directory
fn home() -> PathBuf {
    home_dir().unwrap()
}
