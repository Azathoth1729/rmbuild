use std::path::{Path, PathBuf};

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
