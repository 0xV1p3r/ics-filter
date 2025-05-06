mod version_control;

use clap::Parser;
use config::Config;
use diff;
use git2::Repository;
use icalendar::{Calendar, CalendarComponent, Component};
use reqwest;
use serde::Deserialize;
use serde_json;
use std::fs;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct AppConfig {
    calendar_file: String,
    domain: Option<String>,
    enable_remote: bool,
    remote_name: Option<String>,
    repo_path: String,
    serving_directory: String,
    username: Option<String>,
    token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AppCalendar {
    blacklist: Vec<String>,
    name: String,
    url: String,
}

#[derive(Parser)]
struct CLIArguments {
    config_file: String,
}

fn fetch_calendar(url: &String) -> String {
    // TODO: Better error handling (Result & anyhow)
    let response = reqwest::blocking::get(url).expect("Failed fetching calendar!");
    let body = response.text().expect("Invalid response body!");
    body
}

fn compare_calendars(calendar1: &String, calendar2: &String) -> bool {
    for diff in diff::lines(calendar1, calendar2) {
        match diff {
            diff::Result::Both(_, _) => (),
            diff::Result::Left(_) => return false,
            diff::Result::Right(_) => return false,
        }
    }
    true
}

fn build_filtered_calendar(calendar: &AppCalendar, config: &AppConfig) {
    // TODO: Better error handling (Result & anyhow)
    let raw_path = format!("{}/{}.ics", config.repo_path, calendar.name);
    let path = Path::new(&raw_path);

    let data = read_to_string(path).unwrap();
    let parsed_calendar: Calendar = data.parse().unwrap();

    let mut filtered_calendar = Calendar::new();

    'outer: for component in &parsed_calendar.components {
        if let CalendarComponent::Event(event) = component {
            let summary = event.get_summary().unwrap();
            for entry in &calendar.blacklist {
                if summary == entry {
                    continue 'outer;
                }
            }
            filtered_calendar.push(event.clone());
        }
    }

    //filtered_calendar.name(&parsed_calendar.get_name().unwrap());
    fs::write(
        format!("{}/{}_filtered.ics", config.repo_path, calendar.name),
        format!("{}", filtered_calendar),
    )
    .unwrap()
}

fn load_repo(config: &AppConfig) -> Repository {
    // TODO: Better error handling (Result & anyhow)
    let repo;
    if !Path::new(&config.repo_path).exists() {
        if !config.enable_remote {
            fs::create_dir(&config.repo_path).unwrap();
            repo = Repository::init(&config.repo_path).unwrap();
        } else {
            let url = format!(
                "https://{}@{}/{}/{}.git",
                config.token.clone().unwrap(),
                config.domain.clone().unwrap(),
                config.username.clone().unwrap(),
                config.remote_name.clone().unwrap()
            );
            repo = Repository::clone(&url, &config.repo_path).unwrap();
        }
    } else {
        repo = Repository::open(&config.repo_path).unwrap();
    }
    repo
}

fn load_config(file: &String) -> AppConfig {
    // TODO: Better error handling (Result & anyhow)
    let config = Config::builder()
        .add_source(config::File::with_name(&file))
        .build();
    if let Err(e) = config {
        panic!("{e}");
    }
    let config = config.unwrap().try_deserialize::<AppConfig>();
    config.unwrap()
}

fn load_calendars(file: &String) -> Vec<AppCalendar> {
    // TODO: Better error handling (Result & anyhow)
    let raw_data = read_to_string(file).expect("Unable to open calendar file!");
    serde_json::from_str(&raw_data).expect("Unable to parse calendar file!")
}

fn pipeline(calendar: &AppCalendar, config: &AppConfig) {
    let remote_data = fetch_calendar(&calendar.url);
    let raw_path = format!("{}/{}.ics", config.repo_path, calendar.name);
    let file_path = Path::new(&raw_path);

    if !file_path.exists() {
        fs::write(file_path, &remote_data).expect("Unable to write file!");
        build_filtered_calendar(&calendar, config);
        return;
    }

    let local_data = read_to_string(file_path).expect("Unable to read local calendar data!");

    if compare_calendars(&local_data, &remote_data) {
        return;
    }

    fs::write(file_path, &remote_data).expect("Unable to write file!");
    build_filtered_calendar(&calendar, config);
}

fn refresh_serving_directory(config: &AppConfig) {
    // TODO: Better error handling (Result & anyhow)
    let paths = fs::read_dir(config.repo_path.clone()).unwrap();
    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();
        let extension = path.extension();

        if extension == None {
            continue;
        }

        if extension.unwrap().to_str() == Some("ics") {
            let filename = entry.file_name();
            if filename.to_str().unwrap().ends_with("_filtered.ics") {
                fs::copy(
                    path,
                    format!(
                        "{}/{}.ics",
                        config.serving_directory,
                        filename.to_str().unwrap()
                    ),
                )
                .expect("Unable to copy file!");
            }
        }
    }
}

fn check_repo_for_changes(repository: &Repository) -> bool {
    let statuses = repository.statuses(None).unwrap();
    if statuses.is_empty() { false } else { true }
}

fn commit_repo_changes(repository: &Repository) {
    version_control::add_all(repository);
    if version_control::check_if_no_commits_exist(repository) {
        version_control::create_initial_commit(repository);
    } else {
        version_control::commit("AUTOMATED COMMIT", repository);
    }
}

fn main() {
    let args = CLIArguments::parse();
    let config = load_config(&args.config_file);
    let calendars = load_calendars(&config.calendar_file);
    let repo = load_repo(&config);

    for calendar in calendars {
        print!("Running pipeline for '{}'...", calendar.name);
        pipeline(&calendar, &config);
        print!(" done.\n");
    }

    if check_repo_for_changes(&repo) {
        print!("Committing changes...");
        commit_repo_changes(&repo);
        if config.enable_remote {
            version_control::push_to_remote(
                &repo,
                &config.username.clone().unwrap(),
                &config.token.clone().unwrap(),
            );
        }
        refresh_serving_directory(&config);
        print!(" done.\n");
    }
}
