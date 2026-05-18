use anyhow::{Context, Result};
use icalendar::{Calendar, CalendarComponent, Component, Event, EventLike};
use prettytable::{Table, row};
use regex::Regex;
use similar::{ChangeTag, TextDiff};
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

static TIMESTAMP_REGEX: &str = r"DTSTAMP:\d{8}T\d{6}Z";

#[derive(Default)]
struct CalendarDiff {
    deletions: Vec<Event>,
    insertions: Vec<Event>,
    modifications: Vec<(Event, Event)>,
}

#[derive(Clone, Copy, PartialEq)]
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

struct EventDiff<'a> {
    field_diff: Vec<(EventField, ChangeType)>,
    new: &'a Event,
    old: &'a Event,
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

impl<'a> EventDiff<'a> {
    fn to_string_table(&self) -> Result<String> {
        let comparison_result = stringify::event_diff_to_comparison_rows(self)?;
        let mut event_fields = comparison_result.0;
        let evt_field_mod_tracker = comparison_result.1;

        stringify::insert_unmodified_event_fields(
            self.old,
            &mut event_fields,
            evt_field_mod_tracker,
        )?;

        let mut table = Table::new();

        for field in event_fields {
            table.add_row(row![
                field[0].as_str(),
                field[1].as_str(),
                field[2].as_str(),
            ]);
        }

        Ok(table.to_string())
    }

    fn to_string(&self) -> Result<String> {
        let comparison_result = stringify::event_diff_to_comparison_rows(self)?;
        let mut event_fields = comparison_result.0;
        let evt_field_mod_tracker = comparison_result.1;

        stringify::insert_unmodified_event_fields(
            self.old,
            &mut event_fields,
            evt_field_mod_tracker,
        )?;

        let mut result = String::new();

        for field in event_fields {
            let row = if field[1] == field[2] {
                format!("{}: {}\n", field[0], field[1])
            } else {
                format!("{}: {} -> {}\n", field[0], field[1], field[2])
            };

            result.push_str(&row);
        }

        Ok(result)
    }
}

fn diff_calendars(old: &Calendar, new: &Calendar) -> Result<CalendarDiff> {
    let (mut old_uids, old_events) = map_events(old)?;
    let (mut new_uids, new_events) = map_events(new)?;
    old_uids.append(&mut new_uids);

    let uids: HashSet<&str> = old_uids.into_iter().collect();

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

fn diff_events<'a>(old: &'a Event, new: &'a Event) -> EventDiff<'a> {
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

    EventDiff {
        field_diff: result,
        new,
        old,
    }
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

pub fn generate_diff_report(old: &Calendar, new: &Calendar, as_table: bool) -> Result<DiffReport> {
    let mut report = DiffReport::default();
    let diff = diff_calendars(old, new)?;

    for deletion in diff.deletions {
        let deletion_str = if as_table {
            stringify::event_to_string_table(&deletion)?
        } else {
            stringify::event_to_string(&deletion)?
        };

        report.deletions.push(deletion_str);
    }

    for insertion in diff.insertions {
        let insertion_str = if as_table {
            stringify::event_to_string_table(&insertion)?
        } else {
            stringify::event_to_string(&insertion)?
        };

        report.insertions.push(insertion_str);
    }

    for modifications in diff.modifications {
        let event_diff = diff_events(&modifications.0, &modifications.1);

        let mod_str = if as_table {
            event_diff.to_string_table()?
        } else {
            event_diff.to_string()?
        };

        report.modifications.push(mod_str);
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

mod stringify {
    use super::{ChangeType, Event, EventDiff, EventField};
    use anyhow::Result;
    use chrono::NaiveDateTime;
    use icalendar::{Component, DatePerhapsTime, EventLike};
    use prettytable::{Table, row};

    static MAX_CELL_WIDTH: usize = 40;
    static EVENT_FIELD_STR: [&str; 6] = ["", "start", "end", "location", "priority", "description"];

    fn date_to_str(date: &DatePerhapsTime) -> Result<(String, String)> {
        let raw = date.to_property("START");
        let raw = raw.value();
        let date_time = NaiveDateTime::parse_from_str(raw, "%Y%m%dT%H%M%S")?;

        let date = date_time.format("%d.%m.%Y").to_string();
        let time = date_time.format("%H:%M").to_string();

        Ok((date, time))
    }

    pub fn event_diff_to_comparison_rows(
        event_diff: &EventDiff,
    ) -> Result<(Vec<[String; 3]>, [bool; 6])> {
        let mut rows = vec![[const { String::new() }; 3]; 6];
        let mut evt_field_mod_tracker = [false; 6];

        for diff in &event_diff.field_diff {
            let idx = match diff.0 {
                EventField::DateEnd => 2,
                EventField::DateStart => 1,
                EventField::Description => 5,
                EventField::Location => 3,
                EventField::Priority => 4,
                EventField::Summary => 0,
            };

            let field_str = EVENT_FIELD_STR[idx];
            evt_field_mod_tracker[idx] = true;
            let diff_type = diff.1;

            rows[idx] = match diff_type {
                ChangeType::Deletion => {
                    let value = extract_evt_field_as_str(&diff.0, event_diff.old)?;
                    [field_str.to_string(), value, "None".to_string()]
                }
                ChangeType::Insertion => {
                    let value = extract_evt_field_as_str(&diff.0, event_diff.new)?;
                    [field_str.to_string(), "None".to_string(), value]
                }
                ChangeType::Modification => {
                    let old_value = extract_evt_field_as_str(&diff.0, event_diff.old)?;
                    let new_value = extract_evt_field_as_str(&diff.0, event_diff.new)?;
                    [field_str.to_string(), old_value, new_value]
                }
            };
        }

        Ok((rows, evt_field_mod_tracker))
    }

    fn extract_evt_field_as_str(event_field: &EventField, event: &Event) -> Result<String> {
        let value = match event_field {
            EventField::DateEnd => {
                let (end_date, end_time) = date_to_str(&event.get_end().unwrap())?;
                format!("{end_date} {end_time}")
            }

            EventField::DateStart => {
                let (start_date, start_time) = date_to_str(&event.get_start().unwrap())?;
                format!("{start_date} {start_time}")
            }

            EventField::Description => {
                let raw_str = event.get_description().unwrap().to_string();
                trim_description(raw_str.as_str())
            }

            EventField::Location => event.get_location().unwrap().to_string(),
            EventField::Priority => format!("{}", event.get_priority().unwrap()),
            EventField::Summary => event.get_summary().unwrap().to_string(),
        };

        Ok(value)
    }

    pub fn event_to_string(event: &Event) -> Result<String> {
        let fields = get_event_fields(event)?;

        let mut result = String::new();

        for field in fields {
            result.push_str(&field);
            result.push('\n');
        }

        Ok(result)
    }

    pub fn event_to_string_table(event: &Event) -> Result<String> {
        let [summary, date, start, end, location, priority, description] = get_event_fields(event)?;

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

    fn get_event_fields(event: &Event) -> Result<[String; 7]> {
        let summary = String::from(event.get_summary().unwrap_or("No Heading"));
        let (date, start) = match event.get_start() {
            Some(d) => date_to_str(&d)?,
            None => ("None".to_string(), "None".to_string()),
        };
        let (_, end) = match event.get_end() {
            Some(d) => date_to_str(&d)?,
            None => ("None".to_string(), "None".to_string()),
        };
        let location = String::from(event.get_location().unwrap_or("None"));
        let priority = match event.get_priority() {
            Some(p) => p.to_string(),
            None => "None".to_string(),
        };
        let description = event.get_description().unwrap_or("None");
        let description = trim_description(description);

        Ok([summary, date, start, end, location, priority, description])
    }

    pub fn insert_unmodified_event_fields(
        event: &Event,
        event_fields: &mut Vec<[String; 3]>,
        event_field_mod_tracker: [bool; 6],
    ) -> Result<()> {
        for (idx, field) in event_field_mod_tracker.iter().enumerate() {
            if *field {
                continue;
            }

            let field_str = EVENT_FIELD_STR[idx];
            let event_field = match idx {
                0 => Some(EventField::Summary),
                1 => Some(EventField::DateStart),
                2 => Some(EventField::DateEnd),
                3 => Some(EventField::Location),
                4 => Some(EventField::Priority),
                5 => Some(EventField::Description),
                _ => None,
            };

            let value = extract_evt_field_as_str(&event_field.unwrap(), event)?;
            event_fields[idx] = [field_str.to_string(), value.clone(), value.clone()];
        }

        Ok(())
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
}
