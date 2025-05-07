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

static CALENDAR_FILE: &str = "calendars.json";
static REPO_PATH: &str = "calendar_repo";
static SERVING_DIR: &str = "ics_files";

#[derive(Debug, Deserialize)]
struct AppConfig {
    domain: Option<String>,
    enable_remote: bool,
    remote_name: Option<String>,
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
    #[arg(long)]
    config_file: Option<String>,
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

fn build_filtered_calendar(calendar: &AppCalendar) {
    // TODO: Better error handling (Result & anyhow)
    let raw_path = format!("{}/{}.ics", REPO_PATH, calendar.name);
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

    fs::write(
        format!("{}/{}_filtered.ics", REPO_PATH, calendar.name),
        format!("{}", filtered_calendar),
    )
    .unwrap()
}

fn load_repo(config: &AppConfig) -> Repository {
    // TODO: Better error handling (Result & anyhow)
    let repo;
    if !Path::new(REPO_PATH).exists() {
        if !config.enable_remote {
            fs::create_dir(REPO_PATH).unwrap();
            repo = Repository::init(REPO_PATH).unwrap();
        } else {
            let url = format!(
                "https://{}@{}/{}/{}.git",
                config.token.clone().unwrap(),
                config.domain.clone().unwrap(),
                config.username.clone().unwrap(),
                config.remote_name.clone().unwrap()
            );
            repo = Repository::clone(&url, REPO_PATH).unwrap();
        }
    } else {
        repo = Repository::open(REPO_PATH).unwrap();
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

fn load_calendars(file: &str) -> Vec<AppCalendar> {
    // TODO: Better error handling (Result & anyhow)
    let raw_data = read_to_string(file).expect("Unable to open calendar file!");
    serde_json::from_str(&raw_data).expect("Unable to parse calendar file!")
}

fn pipeline(calendar: &AppCalendar) {
    let remote_data = fetch_calendar(&calendar.url);
    let raw_path = format!("{}/{}.ics", REPO_PATH, calendar.name);
    let file_path = Path::new(&raw_path);

    if !file_path.exists() {
        fs::write(file_path, &remote_data).expect("Unable to write file!");
        build_filtered_calendar(&calendar);
        return;
    }

    let local_data = read_to_string(file_path).expect("Unable to read local calendar data!");

    if compare_calendars(&local_data, &remote_data) {
        return;
    }

    fs::write(file_path, &remote_data).expect("Unable to write file!");
    build_filtered_calendar(&calendar);
}

fn refresh_serving_directory() {
    // TODO: Better error handling (Result & anyhow)
    let paths = fs::read_dir(REPO_PATH).unwrap();
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
                    format!("{}/{}.ics", SERVING_DIR, filename.to_str().unwrap()),
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
    let config = if args.config_file.is_some() {
        load_config(&args.config_file.unwrap())
    } else {
        AppConfig {
            enable_remote: false,
            domain: None,
            remote_name: None,
            username: None,
            token: None,
        }
    };
    let calendars = load_calendars(CALENDAR_FILE);
    let repo = load_repo(&config);

    for calendar in calendars {
        print!("Running pipeline for '{}'...", calendar.name);
        pipeline(&calendar);
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
        refresh_serving_directory();
        print!(" done.\n");
    }
}
