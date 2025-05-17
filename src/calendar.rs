use crate::cache::{is_cached, load_from_cache, save_to_cache};
use crate::config::{CalendarConfig, Config};
use crate::diff::{DiffReport, generate_diff_report, raw_ics_identical};

use anyhow::{Context, Result, bail};
use icalendar::{Calendar, CalendarComponent, Component};
use reqwest::blocking::get;
use url::Url;

struct AppCalendar {
    blacklist: Vec<String>,
    name: String,
    url: Url,
}

fn build_filtered_calendar(calendar: &AppCalendar) -> Result<()> {
    let filename = format!("{}.ics", calendar.name);
    if !is_cached(&filename) {
        bail!("Calendar '{}' is not cached!", calendar.name)
    }
    let data = load_from_cache(&filename)?;

    // Not using with_context() because "the trait bound `std::string::String: StdError` is not satisfied"
    let parsed_calendar: Calendar = match data.parse() {
        Ok(data) => data,
        Err(e) => bail!("Failed to parse calendar '{}'!\n{e}", calendar.name),
    };

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

    save_to_cache(
        &filtered_calendar.to_string(),
        &format!("{}_filtered.ics", calendar.name),
    )
    .with_context(|| "Failed to save filtered calendar to cache!")?;

    Ok(())
}

fn calendar_from_config(calendar_config: &CalendarConfig) -> Result<AppCalendar> {
    let name = if calendar_config.name == None {
        get_calendar_name(&calendar_config.url)?
    } else {
        calendar_config.name.clone().unwrap()
    };
    Ok(AppCalendar {
        blacklist: calendar_config.blacklist.clone(),
        name,
        url: calendar_config.url.clone(),
    })
}

fn fetch_calendar(url: &Url) -> Result<String> {
    let response =
        get(url.clone()).with_context(|| format!("Failed to fetch calendar from '{url}'!"))?;
    response
        .text()
        .with_context(|| "Failed to decode response body!")
}

fn get_calendar_name(url: &Url) -> Result<String> {
    let segments = url
        .path_segments()
        .context("Failed to parse URL segments!")?;
    let last_segment = segments.last().context("Failed to get last URL segment!")?;
    let name = if last_segment.contains('.') {
        last_segment
            .split('.')
            .next()
            .context("Failed to split last URL segment!")?
    } else {
        last_segment
    };
    Ok(name.to_string())
}

fn pipeline_for_calendar(calendar_config: &CalendarConfig) -> Result<Option<(String, DiffReport)>> {
    let calendar = calendar_from_config(calendar_config)?;
    let ics_filename = format!("{}.ics", calendar.name);
    let raw_ics = fetch_calendar(&calendar.url)?;

    if !is_cached(&ics_filename) {
        save_to_cache(&raw_ics, &ics_filename)?;
        build_filtered_calendar(&calendar)?;
        return Ok(None);
    }

    let raw_ics_cached = load_from_cache(&ics_filename)?;

    if raw_ics_identical(&raw_ics_cached, &raw_ics)? {
        return Ok(None);
    }

    build_filtered_calendar(&calendar)?;
    save_to_cache(&raw_ics, &ics_filename)?;

    // Not using with_context() because "the trait bound `std::string::String: StdError` is not satisfied"
    let old_calendar: Calendar = match raw_ics_cached.parse() {
        Ok(data) => data,
        Err(e) => bail!("Failed to parse calendar '{}'!\n{e}", calendar.name),
    };
    // Not using with_context() because "the trait bound `std::string::String: StdError` is not satisfied"
    let new_calendar: Calendar = match raw_ics.parse() {
        Ok(data) => data,
        Err(e) => bail!("Failed to parse calendar '{}'!\n{e}", calendar.name),
    };

    Ok(Some((
        calendar.name,
        generate_diff_report(&old_calendar, &new_calendar)?,
    )))
}

pub fn run_pipeline(config: &Config) -> Result<(Vec<String>, Vec<DiffReport>)> {
    let mut reports = Vec::new();
    let mut updated_names = Vec::new();

    for calendar in &config.calendars {
        match pipeline_for_calendar(calendar)? {
            Some(result) => {
                reports.push(result.1);
                updated_names.push(result.0)
            }
            None => (),
        }
    }

    Ok((updated_names, reports))
}
