mod version_control;

use config::Config;
use git2::Repository;
use icalendar::{Calendar, CalendarComponent, Component};
use regex::Regex;
use reqwest;
use serde::Deserialize;
use serde_json;
use similar::{ChangeTag, TextDiff};
use std::{env, fs, path::Path};

static CALENDAR_FILE: &str = "calendars.json";
static GOTIFY_ENV_VAR_TOKEN: &str = "GOTIFY_APP_TOKEN";
static GOTIFY_ENV_VAR_URL: &str = "GOTIFY_APP_URL";
static REPO_PATH: &str = "calendar_repo";
static SERVING_DIR: &str = "ics_files";
static TIMESTAMP_REGEX: &str = r"DTSTAMP:\d{8}T\d{6}Z";

#[derive(Debug, Deserialize)]
struct AppConfig {
    domain: String,
    email: String,
    remote_enabled: bool,
    remote_name: String,
    username: String,
    token: String,
}

#[derive(Debug, Deserialize)]
struct AppCalendar {
    blacklist: Vec<String>,
    name: String,
    url: String,
}

fn fetch_calendar(url: &String) -> String {
    // TODO: Better error handling (Result & anyhow)
    let response = reqwest::blocking::get(url).expect("Failed fetching calendar!");
    let body = response.text().expect("Invalid response body!");
    body
}

fn calendars_identical(calendar1: &String, calendar2: &String) -> bool {
    let regex = Regex::new(TIMESTAMP_REGEX).unwrap();
    for diff in TextDiff::from_lines(calendar1, calendar2).iter_all_changes() {
        if diff.tag() == ChangeTag::Equal {
            continue;
        }
        if !regex.is_match(&diff.to_string()) {
            return false;
        }
    }
    true
}

fn build_filtered_calendar(calendar: &AppCalendar) {
    // TODO: Better error handling (Result & anyhow)
    let raw_path = format!("{}/{}.ics", REPO_PATH, calendar.name);
    let path = Path::new(&raw_path);

    let data = fs::read_to_string(path).unwrap();
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
    match Repository::open(REPO_PATH) {
        Ok(r) => r,
        Err(_) => {
            if !config.remote_enabled {
                Repository::init(REPO_PATH).unwrap()
            } else {
                let url = format!(
                    "https://{}@{}/{}/{}.git",
                    config.token.clone(),
                    config.domain.clone(),
                    config.username.clone(),
                    config.remote_name.clone()
                );
                Repository::clone(&url, REPO_PATH).unwrap()
            }
        }
    }
}

fn load_config() -> AppConfig {
    // TODO: Better error handling (Result & anyhow)
    let config = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build();
    if let Err(e) = config {
        panic!("{e}");
    }
    let config = config.unwrap().try_deserialize::<AppConfig>();
    config.unwrap()
}

fn load_calendars(file: &str) -> Vec<AppCalendar> {
    // TODO: Better error handling (Result & anyhow)
    let raw_data = fs::read_to_string(file).expect("Unable to open calendar file!");
    serde_json::from_str(&raw_data).expect("Unable to parse calendar file!")
}

fn pipeline(calendar: &AppCalendar) -> bool {
    let remote_data = fetch_calendar(&calendar.url);
    let raw_path = format!("{}/{}.ics", REPO_PATH, calendar.name);
    let file_path = Path::new(&raw_path);

    if !file_path.exists() {
        fs::write(file_path, &remote_data).expect("Unable to write file!");
        build_filtered_calendar(&calendar);
        return true;
    }

    let local_data = fs::read_to_string(file_path).expect("Unable to read local calendar data!");

    if calendars_identical(&local_data, &remote_data) {
        return false;
    }

    fs::write(file_path, &remote_data).expect("Unable to write file!");
    build_filtered_calendar(&calendar);
    true
}

fn refresh_serving_directory() {
    // TODO: Better error handling (Result & anyhow)
    if !Path::new(SERVING_DIR).exists() {
        match fs::create_dir(SERVING_DIR) {
            Ok(_) => (),
            Err(e) => panic!("{e}"),
        }
    }
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
                let filename = filename.to_str().unwrap().strip_suffix("_filtered.ics");
                fs::copy(path, format!("{}/{}.ics", SERVING_DIR, filename.unwrap()))
                    .expect("Unable to copy file!");
            }
        }
    }
}

fn check_repo_for_changes(repository: &Repository) -> bool {
    let statuses = repository.statuses(None).unwrap();
    if statuses.is_empty() { false } else { true }
}

fn commit_repo_changes(config: &AppConfig, repository: &Repository) {
    version_control::add_all(repository);
    let signature = git2::Signature::now(&config.username, &config.email).unwrap();
    if version_control::check_if_no_commits_exist(repository) {
        version_control::create_initial_commit(repository, signature);
    } else {
        version_control::commit("AUTOMATED COMMIT", repository, signature);
    }
}

fn notify() {
    let token = match env::var(GOTIFY_ENV_VAR_TOKEN) {
        Ok(var) => var,
        Err(_) => return,
    };
    let url = match env::var(GOTIFY_ENV_VAR_URL) {
        Ok(var) => var,
        Err(_) => return,
    };

    let params = [("title", "Update"), ("message", "A Change occurred!")];
    let url = format!("{}/message?token={}", url, token);
    let client = reqwest::blocking::Client::new();
    let resp = client.post(url).form(&params).send();

    if resp.is_err() {
        eprintln!("Failed to send message!");
    } else {
        println!("{:?}", resp.unwrap().text())
    }
}

fn main() {
    let calendars = load_calendars(CALENDAR_FILE);
    let config_exists = Path::new("config.toml").exists();

    if !Path::new(REPO_PATH).exists() {
        if config_exists {
            // Initialize Git Repo - Either clone or init
            let config = load_config();
            load_repo(&config);
        } else {
            // Create a simple directory
            match fs::create_dir(REPO_PATH) {
                Ok(_) => (),
                Err(e) => panic!("{e}"),
            }
        }
    }

    let mut changes_occurred = false;
    for calendar in calendars {
        print!("Running pipeline for '{}'...", calendar.name);
        let result = pipeline(&calendar);
        print!(" done.\n");
        if result {
            changes_occurred = true;
            println!("Changes detected.")
        } else {
            println!("No changes detected.")
        }
    }

    if config_exists && changes_occurred {
        let config = load_config();
        let repo = load_repo(&config);
        if check_repo_for_changes(&repo) {
            print!("Committing changes...");
            commit_repo_changes(&config, &repo);
            if config.remote_enabled {
                version_control::push_to_remote(
                    &repo,
                    &config.username.clone(),
                    &config.token.clone(),
                );
            }
            refresh_serving_directory();
            print!(" done.\n");
        }
    } else if changes_occurred {
        refresh_serving_directory();
    }

    if changes_occurred {
        notify();
    }
}
