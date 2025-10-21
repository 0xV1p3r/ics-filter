use crate::cache::{copy_from_cache, initialize_cache};
use crate::calendar::run_pipeline;
use crate::config::{load_config, Config};
use crate::git_repo::{initialize_repo, update_repo};
use crate::notification::push_notifications;

use anyhow::{Context, Result};
use chrono::Local;
use std::fs::{create_dir, File};
use std::path::Path;

mod cache;
mod calendar;
mod config;
mod diff;
mod git_repo;
mod notification;

static INIT_MARKER: &str = ".initialized";
static SERVING_DIRECTORY: &str = "calendar_serving";

fn already_initialized() -> bool {
    Path::new(INIT_MARKER).exists()
}

fn initialize(config: &Config) -> Result<()> {
    if config.git.is_some() {
        let git_cfg = config.clone().git.unwrap();
        initialize_repo(&git_cfg)?;
    }

    initialize_cache(config)?;

    File::create(INIT_MARKER).with_context(|| "Failed to create init marker file")?;
    Ok(())
}

fn update_serving_directory(filenames: &Vec<String>) -> Result<()> {
    if !Path::new(SERVING_DIRECTORY).exists() {
        create_dir(SERVING_DIRECTORY)?;
    }

    for filename in filenames {
        let src = format!("{filename}_filtered.ics");
        let dest = format!("{SERVING_DIRECTORY}/{filename}.ics");
        copy_from_cache(&src, &dest)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let config = load_config()?;

    if !already_initialized() {
        println!(
            "[{}] Initializing...",
            Local::now().format("%Y-%m-%dT%H:%M:%S")
        );
        initialize(&config)?;
    }

    println!(
        "[{}] Running pipeline...",
        Local::now().format("%Y-%m-%dT%H:%M:%S")
    );
    let (updated_files, reports) = run_pipeline(&config)?;

    if updated_files.is_empty() {
        println!(
            "[{}] No changes detected.",
            Local::now().format("%Y-%m-%dT%H:%M:%S")
        );
        return Ok(());
    }

    println!(
        "[{}] Changes detected.",
        Local::now().format("%Y-%m-%dT%H:%M:%S")
    );

    println!(
        "[{}] Updating serving directory.",
        Local::now().format("%Y-%m-%dT%H:%M:%S")
    );
    update_serving_directory(&updated_files)?;
    push_notifications(&config, &updated_files, reports)?;

    if config.git.is_some() {
        let git_cfg = config.git.unwrap();
        println!(
            "[{}] Updating git repo.",
            Local::now().format("%Y-%m-%dT%H:%M:%S")
        );
        update_repo(&updated_files, git_cfg)?;
    }

    Ok(())
}
