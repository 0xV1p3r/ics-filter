use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::{fs::read_to_string, path::Path};
use url::Url;

static CONFIG_FILE: &str = "config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub calendars: Vec<CalendarConfig>,
    pub git: Option<GitConfig>,
    pub gotify: Option<GotifyConfig>,
    pub refresh_interval: u8,
}

#[derive(Deserialize)]
pub struct CalendarConfig {
    pub blacklist: Vec<String>,
    pub name: Option<String>,
    pub url: Url,
}

#[derive(Clone, Deserialize)]
pub struct GotifyConfig {
    pub server: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct GitConfig {
    pub remote: Option<GitRemoteConfig>,
    pub signature: GitSignatureConfig,
}

#[derive(Deserialize)]
pub struct GitRemoteConfig {
    pub repository: String,
    pub token: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct GitSignatureConfig {
    pub email: String,
    pub username: String,
}

pub fn load_config() -> Result<Config> {
    if !Path::new(CONFIG_FILE).exists() {
        bail!("Config file '{CONFIG_FILE}' not found!")
    }

    let data = read_to_string(CONFIG_FILE)?;
    toml::from_str(&data).context("Failed to parse config!")
}
