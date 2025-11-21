use chrono::{DateTime, Utc};
use diesel::prelude::*;

use super::{super::calendar::Calendar, Event};

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(
    belongs_to(Calendar),
    belongs_to(Event),
    table_name = crate::schema::event_snapshots,
    treat_none_as_default_value = false
)]
pub struct EventSnapshot {
    pub id: i32,
    pub calendar_id: i32,
    pub event_id: i32,
    pub timestamp: DateTime<Utc>,
    pub uid: String,
    pub summary: String,
    pub location: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
}
