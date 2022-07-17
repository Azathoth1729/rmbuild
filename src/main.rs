use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use colored::*;
use home::home_dir;

use rmbuild::cargo_targets;

fn home() -> PathBuf {
    home_dir().unwrap()
}

fn workshop() -> PathBuf {
    let mut home = home();
    home.push("workshop");
    home
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// start path for rmbuild, default is your home directory
    #[clap(value_parser)]
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let root_path = args.path.unwrap_or(workshop());

    println!(
        "{}",
        format!("starting rmbuild from {:?}", root_path).cyan()
    );

    let packages = cargo_targets(root_path);
    let mut it = packages.iter();
   
    println!("{}", "find the following packages:".green());
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
                    // fs::remove_dir_all(tar)?;
                    println!("{}", "removed".green());
                    break;
                }

                "n" | "no" => {
                    break;
                }

                "all" => {
                    println!("{}", "remove the following: ".red());
                    println!("{}", format!("{}", cur_tar.display()).green());
                    // fs::remove_dir_all(cur_tar)?;

                    for tar in it {
                        println!("{}", format!("{}", tar.display()).green());
                        // fs::remove_dir_all(tar)?;
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
