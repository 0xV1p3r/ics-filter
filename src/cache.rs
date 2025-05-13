use crate::{config::Config, git_repo::REPO_PATH};

use anyhow::{Context, Result, bail};
use std::fs::{copy, create_dir, read_dir, read_to_string, write};
use std::path::Path;

static CACHE_DIR: &str = "cache";

pub fn copy_from_cache(filename: &String, destination: &str) -> Result<()> {
    let src_raw = format!("{CACHE_DIR}/{filename}");
    let src = Path::new(&src_raw);
    if !src.exists() {
        bail!("File '{filename}' does not exist!")
    }
    copy(src, destination)?;

    Ok(())
}

pub fn initialize_cache(config: &Config) -> Result<()> {
    if !Path::new(CACHE_DIR).exists() {
        create_dir(CACHE_DIR).with_context(|| "Failed to create cache directory!")?
    }
    if config.git.is_some() {
        let dir_contents = read_dir(REPO_PATH).with_context(|| "Failed to read repo directory!")?;
        for entry in dir_contents {
            let entry = entry.with_context(|| "Failed to unpack DirEntry!")?;
            let path = entry.path();
            let extension = path.extension();
            if extension.is_none() {
                continue;
            }
            if extension.unwrap().to_str().unwrap() == "ics" {
                copy(path, CACHE_DIR).with_context(|| "Failed to copy file to cache!")?;
            }
        }
    }
    Ok(())
}

pub fn is_cached(filename: &String) -> bool {
    if Path::new(&format!("{CACHE_DIR}/{filename}")).exists() {
        true
    } else {
        false
    }
}

pub fn load_from_cache(filename: &String) -> Result<String> {
    let path_as_str = format!("{CACHE_DIR}/{filename}");
    let path = Path::new(&path_as_str);
    if !path.exists() {
        bail!("File '{filename}' does not exist!")
    }
    Ok(read_to_string(path).with_context(|| format!("Failed to read file '{filename}'!"))?)
}

pub fn save_to_cache(contents: &String, filename: &String) -> Result<()> {
    let path_as_str = format!("{CACHE_DIR}/{filename}");
    let path = Path::new(&path_as_str);
    write(&path, contents).with_context(|| format!("Failed to write file '{filename}'!"))?;
    Ok(())
}
