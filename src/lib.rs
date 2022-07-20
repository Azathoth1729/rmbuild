use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;
use colored::*;
use rayon::prelude::*;
use walkdir::DirEntry;

fn is_cargo_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() && entry.path().join("Cargo.toml").is_file()
}

fn is_in_hidden(entry: &DirEntry) -> bool {
    entry.path().ancestors().any(|p| {
        p.file_name()
            .map(|e| e.to_str())
            .flatten()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    })
}

fn is_in_rustup_installation(entry: &DirEntry) -> bool {
    entry
        .path()
        .to_str()
        .map(|s| s.contains(".cargo") || s.contains(".rustup"))
        .unwrap_or(false)
}

pub fn cargo_targets<P>(root: P) -> Vec<PathBuf>
where
    P: AsRef<Path>,
{
    walkdir::WalkDir::new(root)
        .into_iter()
        .par_bridge()
        .filter_map(|file| file.ok())
        .filter(|e| !is_in_rustup_installation(e))
        .filter(|e| !is_in_hidden(e))
        .filter(is_cargo_dir)
        .map(|e| e.path().join("target"))
        .filter(|e| e.is_dir())
        .collect()
}

pub fn visit(root: PathBuf) -> Result<()> {
    println!(
        "{}",
        format!("starting rmbuild from {}...", root.display()).cyan()
    );

    let packages = cargo_targets(root);

    println!("\n{}", "found the following packages:".green());

    packages
        .iter()
        .for_each(|package| println!("{}", format!("{}", package.display()).green()));

    println!("\n{}", CMD_HELP.yellow());

    let mut it = packages.iter();
    while let Some(tar) = it.next() {
        let cur_tar = &tar;
        print!("remove {}?:", format!("{}", cur_tar.display()).red());
        io::stdout().flush().unwrap();

        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            let s = buffer.trim();
            match s {
                "y" | "yes" => {
                    fs::remove_dir_all(tar)?;
                    println!("{}", "removed".green());
                    break;
                }

                "n" | "no" => {
                    break;
                }

                "all" => {
                    println!("{}", "remove the following: ".red());
                    println!("{}", format!("{}", cur_tar.display()).green());
                    fs::remove_dir_all(cur_tar)?;

                    for tar in it {
                        println!("{}", format!("{}", tar.display()).green());
                        fs::remove_dir_all(tar)?;
                    }

                    return Ok(());
                }

                cmd => {
                    println!(
                        "{}",
                        format!("unknown command {}, please try again", cmd.yellow()).red()
                    );
                }
            }
        }
    }
    Ok(())
}

const CMD_HELP: &str = "please input the following command to remove these targers
yes|y : remove the current target folder
no|n  : do not do anything
all   : remove all the following targets
";
