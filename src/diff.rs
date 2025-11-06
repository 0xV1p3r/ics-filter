use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use icalendar::{Calendar, CalendarComponent, Component, DatePerhapsTime, Event, EventLike};
use prettytable::{Table, row};
use regex::Regex;
use similar::{ChangeTag, TextDiff};
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

static MAX_CELL_WIDTH: usize = 40;
static TIMESTAMP_REGEX: &str = r"DTSTAMP:\d{8}T\d{6}Z";

#[derive(Default)]
struct CalendarDiff {
    deletions: Vec<Event>,
    insertions: Vec<Event>,
    modifications: Vec<(Event, Event)>,
}

#[derive(PartialEq)]
enum ChangeType {
    Deletion,
    Insertion,
    Modification,
}

#[derive(Debug, Default)]
pub struct DiffReport {
    pub deletions: Vec<String>,
    pub insertions: Vec<String>,
    pub modifications: Vec<String>,
}

#[derive(PartialEq)]
enum EventField {
    Description,
    DateEnd,
    DateStart,
    Location,
    Priority,
    Summary,
}

fn date_to_str(date: &DatePerhapsTime) -> Result<(String, String)> {
    let raw = date.to_property("START");
    let raw = raw.value();
    let date_time = NaiveDateTime::parse_from_str(raw, "%Y%m%dT%H%M%S")?;

    let date = date_time.format("%d.%m.%Y").to_string();
    let time = date_time.format("%H:%M").to_string();

    Ok((date, time))
}

fn diff_calendars(old: &Calendar, new: &Calendar) -> Result<CalendarDiff> {
    let (mut old_uids, old_events) = map_events(old)?;
    let (mut new_uids, new_events) = map_events(new)?;
    old_uids.append(&mut new_uids); // Merge old & new uids with duplicates

    // Make a vector of uids containing no duplicates
    let mut uids = Vec::new();
    let mut seen_uids = HashSet::new();
    for uid in old_uids {
        if !seen_uids.contains(uid) {
            seen_uids.insert(uid);
            uids.push(uid);
        }
    }

    let mut calendar_diff = CalendarDiff::default();
    for uid in uids {
        let old = old_events.get(uid);
        let new = new_events.get(uid);

        if let Some(old) = old
            && new.is_none()
        {
            calendar_diff.deletions.push(old.clone());
        } else if let Some(new) = new
            && old.is_none()
        {
            calendar_diff.insertions.push(new.clone());
        } else if !events_identical(old.unwrap(), new.unwrap()) {
            calendar_diff
                .modifications
                .push((old.unwrap().clone(), new.unwrap().clone()));
        }
    }

    Ok(calendar_diff)
}

fn diff_events(old: &Event, new: &Event) -> Vec<(EventField, ChangeType)> {
    let mut result = Vec::new();

    let (old_description, new_description) = (old.get_description(), new.get_description());
    if old_description != new_description {
        result.push((
            EventField::Description,
            if old_description.is_none() {
                ChangeType::Insertion
            } else if new_description.is_none() {
                ChangeType::Deletion
            } else {
                ChangeType::Modification
            },
        ));
    }

    let (old_date_end, new_date_end) = (old.get_end(), new.get_end());
    if old_date_end != new_date_end {
        result.push((
            EventField::DateEnd,
            if old_date_end.is_none() {
                ChangeType::Insertion
            } else if new_date_end.is_none() {
                ChangeType::Deletion
            } else {
                ChangeType::Modification
            },
        ));
    }

    let (old_date_start, new_date_start) = (old.get_start(), new.get_start());
    if old_date_start != new_date_start {
        result.push((
            EventField::DateStart,
            if old_date_start.is_none() {
                ChangeType::Insertion
            } else if new_date_start.is_none() {
                ChangeType::Deletion
            } else {
                ChangeType::Modification
            },
        ));
    }

    let (old_location, new_location) = (old.get_location(), new.get_location());
    if old_location != new_location {
        result.push((
            EventField::Location,
            if old_location.is_none() {
                ChangeType::Insertion
            } else if new_location.is_none() {
                ChangeType::Deletion
            } else {
                ChangeType::Modification
            },
        ));
    }

    let (old_priority, new_priority) = (old.get_priority(), new.get_priority());
    if old_priority != new_priority {
        result.push((
            EventField::Priority,
            if old_priority.is_none() {
                ChangeType::Insertion
            } else if new_priority.is_none() {
                ChangeType::Deletion
            } else {
                ChangeType::Modification
            },
        ));
    }

    let (old_summary, new_summary) = (old.get_summary(), new.get_summary());
    if old_summary != new_summary {
        result.push((
            EventField::Summary,
            if old_summary.is_none() {
                ChangeType::Insertion
            } else if new_summary.is_none() {
                ChangeType::Deletion
            } else {
                ChangeType::Modification
            },
        ));
    }

    result
}

fn event_diff_to_str(
    diffs: Vec<(EventField, ChangeType)>,
    old: &Event,
    new: &Event,
) -> Result<String> {
    let mut fields = vec![[const { String::new() }; 3]; 6];
    let mut track_fields = [false; 6];
    let field_strings = ["", "start", "end", "location", "priority", "description"];

    for diff in diffs {
        let idx = match diff.0 {
            EventField::DateEnd => 2,
            EventField::DateStart => 1,
            EventField::Description => 5,
            EventField::Location => 3,
            EventField::Priority => 4,
            EventField::Summary => 0,
        };
        let field_str = field_strings[idx];
        track_fields[idx] = true;
        if diff.1 == ChangeType::Insertion {
            let mut value = event_field_to_str(&diff.0, new)?;

            if diff.0 == EventField::Description {
                value = trim_description(value.as_str());
            }

            fields[idx] = [field_str.to_string(), "None".to_string(), value];
        } else if diff.1 == ChangeType::Deletion {
            let mut value = event_field_to_str(&diff.0, old)?;

            if diff.0 == EventField::Description {
                value = trim_description(value.as_str());
            }

            fields[idx] = [field_str.to_string(), value, "None".to_string()];
        } else {
            let mut old_value = event_field_to_str(&diff.0, old)?;
            let mut new_value = event_field_to_str(&diff.0, new)?;

            if diff.0 == EventField::Description {
                old_value = trim_description(old_value.as_str());
                new_value = trim_description(new_value.as_str());
            }

            fields[idx] = [field_str.to_string(), old_value, new_value];
        }
    }

    for (idx, field) in track_fields.iter().enumerate() {
        if *field {
            continue;
        }
        let field_str = field_strings[idx];
        let event_field = match idx {
            0 => Some(EventField::Summary),
            1 => Some(EventField::DateStart),
            2 => Some(EventField::DateEnd),
            3 => Some(EventField::Location),
            4 => Some(EventField::Priority),
            5 => Some(EventField::Description),
            _ => None,
        };
        let value = event_field_to_str(&event_field.unwrap(), old)?;
        fields[idx] = [field_str.to_string(), value.clone(), value.clone()];
    }

    let mut table = Table::new();
    for field in fields {
        table.add_row(row![
            field[0].as_str(),
            field[1].as_str(),
            field[2].as_str(),
        ]);
    }

    Ok(table.to_string())
}

fn event_field_to_str(event_field: &EventField, event: &Event) -> Result<String> {
    let value = match event_field {
        EventField::DateEnd => {
            let (end_date, end_time) = date_to_str(&event.get_end().unwrap())?;
            format!("{end_date} {end_time}")
        }
        EventField::DateStart => {
            let (start_date, start_time) = date_to_str(&event.get_start().unwrap())?;
            format!("{start_date} {start_time}")
        }
        EventField::Description => event.get_description().unwrap().to_string(),
        EventField::Location => event.get_location().unwrap().to_string(),
        EventField::Priority => format!("{}", event.get_priority().unwrap()),
        EventField::Summary => event.get_summary().unwrap().to_string(),
    };
    Ok(value)
}

fn events_identical(event1: &Event, event2: &Event) -> bool {
    let description = event1.get_description() == event2.get_description();
    let date_end = event1.get_end() == event2.get_end();
    let date_start = event1.get_start() == event2.get_start();
    let location = event1.get_location() == event2.get_location();
    let priority = event1.get_priority() == event2.get_priority();
    let summary = event1.get_summary() == event2.get_summary();

    description && date_end && date_start && location && priority && summary
}

fn event_to_str(event: &Event) -> Result<String> {
    // TODO: For logging -> If a field is None emit a warning
    let summary = event.get_summary().unwrap_or("No Heading");
    let (date, start) = match event.get_start() {
        Some(d) => date_to_str(&d)?,
        None => ("None".to_string(), "None".to_string()),
    };
    let (_, end) = match event.get_end() {
        Some(d) => date_to_str(&d)?,
        None => ("None".to_string(), "None".to_string()),
    };
    let location = event.get_location().unwrap_or("None");
    let priority = match event.get_priority() {
        Some(p) => p.to_string(),
        None => "None".to_string(),
    };
    let description = event.get_description().unwrap_or("None");
    let description = trim_description(description);

    let mut table = Table::new();
    table.add_row(row!["", summary]);
    table.add_row(row!["date", date]);
    table.add_row(row!["start", start]);
    table.add_row(row!["end", end]);
    table.add_row(row!["location", location]);
    table.add_row(row!["priority", priority]);
    table.add_row(row!["description", description]);

    table.printstd();
    Ok(table.to_string())
}

pub fn generate_diff_report(old: &Calendar, new: &Calendar) -> Result<DiffReport> {
    let mut report = DiffReport::default();
    let diff = diff_calendars(old, new)?;

    for deletion in diff.deletions {
        report.deletions.push(event_to_str(&deletion)?);
    }
    for insertion in diff.insertions {
        report.insertions.push(event_to_str(&insertion)?);
    }
    for modifications in diff.modifications {
        let event_diff = diff_events(&modifications.0, &modifications.1);
        report.modifications.push(event_diff_to_str(
            event_diff,
            &modifications.0,
            &modifications.1,
        )?);
    }

    Ok(report)
}

fn map_events(calendar: &Calendar) -> Result<(Vec<&str>, HashMap<&str, Event>)> {
    let mut event_uids = Vec::new();
    let mut events = HashMap::new();
    for component in &calendar.components {
        if let CalendarComponent::Event(event) = component {
            let uid = event.get_uid().context("Failed to get event uid!")?;
            event_uids.push(uid);
            events.insert(uid, event.clone());
        }
    }
    Ok((event_uids, events))
}

pub fn raw_ics_identical(old: &str, new: &str) -> Result<bool> {
    let regex =
        Regex::new(TIMESTAMP_REGEX).with_context(|| "Failed to compile regular expression!")?;
    for diff in TextDiff::from_lines(old, new).iter_all_changes() {
        if diff.tag() == ChangeTag::Equal {
            continue;
        }
        if !regex.is_match(&diff.to_string()) {
            // Ignore the 'DTSTAMP' field
            return Ok(false);
        }
    }

    Ok(true)
}

fn trim_description(description: &str) -> String {
    let mut new_description = String::new();

    for line in description.lines() {
        let line_len = line.len();
        let new_line = if line_len > MAX_CELL_WIDTH {
            textwrap::fill(line, MAX_CELL_WIDTH)
        } else {
            line.to_string()
        };
        new_description.push_str(&new_line);
        new_description.push('\n');
    }

    new_description
}
