use crate::config::{Config, GotifyConfig};
use crate::diff::DiffReport;

use anyhow::Result;
use reqwest::blocking::Client;
use url::Url;

fn notifications_configured(config: &Config) -> bool {
    if config.gotify.is_some() {
        return true;
    }

    false
}
fn push_messages_gotify(config: &GotifyConfig, messages: Vec<(String, String)>) -> Result<()> {
    let base = Url::parse(&config.server)?;
    let mut url = base.join("message")?;
    url.set_query(Some(&format!("token={}", &config.token)));
    let url_str = url.to_string();

    let client = Client::new();
    for msg in messages {
        let params = [("title", msg.0), ("message", msg.1)];
        let _ = client.post(&url_str).form(&params).send()?;
    }

    Ok(())
}

pub fn push_notifications(config: &Config, reports: Vec<(String, DiffReport)>) -> Result<()> {
    if !notifications_configured(&config) {
        return Ok(());
    }

    let mut messages = Vec::new();
    for report in reports {
        let calendar_name = report.0;
        let report = report.1;
        let title = format!("{calendar_name} - Event deleted");

        for msg in report.deletions {
            messages.push((title.clone(), msg))
        }

        let title = format!("{calendar_name} - Event added");
        for msg in report.insertions {
            messages.push((title.clone(), msg))
        }

        let title = format!("{calendar_name} - Event modified");
        for msg in report.modifications {
            messages.push((title.clone(), msg))
        }
    }

    if config.gotify.is_some() {
        push_messages_gotify(&config.gotify.clone().unwrap(), messages)?;
    }

    Ok(())
}
