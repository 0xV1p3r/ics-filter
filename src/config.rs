use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::{fs::read_to_string, path::Path};
use url::Url;

static CONFIG_FILE: &str = "config.toml";

#[derive(Clone, Deserialize)]
pub struct Config {
    pub calendars: Vec<CalendarConfig>,
    pub git: Option<GitConfig>,
    pub notifications: Option<NotificationConfig>,
}

#[derive(Clone, Deserialize)]
pub struct CalendarConfig {
    pub blacklist: Vec<String>,
    pub name: Option<String>,
    pub url: Url,
}

#[derive(Clone, Deserialize)]
pub struct EmailConfig {
    pub smtp_server: String,
    pub username: String,
    pub password: String,
    pub recipients: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct GotifyConfig {
    pub server: String,
    pub token: String,
}

#[derive(Clone, Deserialize)]
pub struct GitConfig {
    pub remote: Option<GitRemoteConfig>,
    pub signature: GitSignatureConfig,
}

#[derive(Clone, Deserialize)]
pub struct GitRemoteConfig {
    pub domain: String,
    pub repository: String,
    pub token: String,
    pub username: String,
}

#[derive(Clone, Deserialize)]
pub struct GitSignatureConfig {
    pub email: String,
    pub username: String,
}

#[derive(Clone, Deserialize)]
pub struct NotificationConfig {
    pub email: Option<EmailConfig>,
    pub gotify: Option<GotifyConfig>,
}

pub fn load_config() -> Result<Config> {
    if !Path::new(CONFIG_FILE).exists() {
        bail!("Config file '{CONFIG_FILE}' not found!")
    }

    let data = read_to_string(CONFIG_FILE)?;
    toml::from_str(&data).context("Failed to parse config!")
}
