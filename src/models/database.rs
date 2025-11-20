use chrono::NaiveDateTime;
use diesel::prelude::*;
use make_public::make_public;

use crate::models::{
    color::Color,
    filter::{FilterCriteriaType, FilterType},
};

#[make_public]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::calendars, check_for_backend(diesel::sqlite::Sqlite))]
struct Calendar {
    id: i32,
    name: String,
    url: String,
    color: Color,
}

#[make_public]
#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(
    belongs_to(Calendar),
    table_name = crate::schema::events,
    treat_none_as_default_value = false,
    check_for_backend(diesel::sqlite::Sqlite)
)]
struct Event {
    id: i32,
    calendar_id: i32,
    uid: String,
    summary: String,
    location: String,
    description: Option<String>,
    timezone: String,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
}

#[make_public]
#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(
    belongs_to(Calendar),
    belongs_to(Event),
    table_name = crate::schema::event_snapshots,
    treat_none_as_default_value = false,
    check_for_backend(diesel::sqlite::Sqlite)
)]
struct EventSnapshot {
    id: i32,
    calendar_id: i32,
    event_id: i32,
    timestamp: NaiveDateTime,
    uid: String,
    summary: String,
    location: String,
    description: Option<String>,
    timezone: String,
    start_date: NaiveDateTime,
}

#[make_public]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::filters, check_for_backend(diesel::sqlite::Sqlite))]
struct Filter {
    id: i32,
    filter_type: FilterType,
    name: String,
}

#[make_public]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::filter_criteria, check_for_backend(diesel::sqlite::Sqlite))]
struct FilterCriteria {
    id: i32,
    criteria_type: FilterCriteriaType,
    value: String,
}

#[make_public]
#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(
    belongs_to(Filter),
    belongs_to(FilterCriteria),
    primary_key(filter_id, filter_criteria_id),
    table_name = crate::schema::filter_criteria_filters,
    check_for_backend(diesel::sqlite::Sqlite)
)]
struct FilterCriteriaFilters {
    filter_id: i32,
    filter_criteria_id: i32,
}

#[make_public]
#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(
    belongs_to(Calendar),
    belongs_to(Filter),
    primary_key(filter_id, calendar_id),
    table_name = crate::schema::filtered_calendars,
    check_for_backend(diesel::sqlite::Sqlite)
)]
struct FilteredCalendar {
    calendar_id: i32,
    filter_id: i32,
    name: String,
    color: Color,
}
