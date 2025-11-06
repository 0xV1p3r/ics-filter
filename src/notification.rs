use crate::config::{Config, EmailConfig, GotifyConfig};
use crate::diff::DiffReport;

use anyhow::{Context, Result};
use chrono::Local;
use lettre::message::{Mailbox, Mailboxes, header::ContentType};
use lettre::transport::smtp::{SmtpTransport, authentication::Credentials};
use lettre::{Message, Transport};
use reqwest::blocking::Client;
use url::Url;

fn notifications_configured(config: &Config) -> bool {
    if config.notifications.is_some() {
        return true;
    }

    false
}

fn push_messages_email(config: &EmailConfig, messages: &Vec<(String, String)>) -> Result<()> {
    let recipients = &config.recipients.join(",");
    let from_mailbox: Mailbox = config
        .username
        .parse()
        .with_context(|| "Failed to parse sender email address")?;
    let to_mailboxes: Mailboxes = recipients
        .parse()
        .with_context(|| "Failed to parse recipient email addresses")?;
    let to_header: lettre::message::header::To = to_mailboxes.into();

    let credentials = Credentials::new(config.username.clone(), config.password.clone());
    let mailer = SmtpTransport::relay(&config.smtp_server)?
        .credentials(credentials)
        .build();

    for msg in messages {
        let subject = msg.0.clone();
        let body = msg.1.clone();

        let email = Message::builder()
            .mailbox(to_header.clone())
            .from(from_mailbox.clone())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)
            .with_context(|| "Failed to construct email")?;

        mailer
            .send(&email)
            .with_context(|| "Failed to send email")?;
    }
    Ok(())
}

fn push_messages_gotify(config: &GotifyConfig, messages: &Vec<(String, String)>) -> Result<()> {
    let mut url = Url::parse("https://gotify.net")?;
    url.set_host(Some(&config.server))
        .with_context(|| "Failed to insert configured domain")?;
    url.set_path("message");
    url.set_query(Some(&format!("token={}", &config.token)));
    let url_str = url.to_string();

    let client = Client::new();
    for msg in messages {
        let params = [("title", msg.0.clone()), ("message", msg.1.clone())];
        let _ = client.post(&url_str).form(&params).send()?;
    }

    Ok(())
}

pub fn push_notifications(
    config: &Config,
    names: &[String],
    reports: Vec<DiffReport>,
) -> Result<()> {
    if !notifications_configured(config) {
        return Ok(());
    }

    let notification_config = config.notifications.clone().unwrap();

    let mut messages = Vec::with_capacity(reports.len());

    for (calendar_name, report) in names.iter().zip(reports) {
        let title = format!("'{calendar_name}' -- Event deleted");

        for msg in report.deletions {
            messages.push((title.clone(), msg));
        }

        let title = format!("'{calendar_name}' -- Event added");
        for msg in report.insertions {
            messages.push((title.clone(), msg));
        }

        let title = format!("'{calendar_name}' -- Event modified");
        for msg in report.modifications {
            messages.push((title.clone(), msg));
        }
    }

    if notification_config.email.is_some() {
        println!(
            "[{}] Sending email notifications.",
            Local::now().format("%Y-%m-%dT%H:%M:%S")
        );
        push_messages_email(&notification_config.email.clone().unwrap(), &messages)?;
    }

    if notification_config.gotify.is_some() {
        println!(
            "[{}] Sending gotify notifications.",
            Local::now().format("%Y-%m-%dT%H:%M:%S")
        );
        push_messages_gotify(&notification_config.gotify.clone().unwrap(), &messages)?;
    }

    Ok(())
}
