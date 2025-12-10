use chrono::{DateTime, Utc};
use diesel::prelude::*;

use super::{super::calendar::Calendar, Event};

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(
    belongs_to(Calendar),
    belongs_to(Event, foreign_key = id),
    table_name = crate::schema::event_snapshot,
    treat_none_as_default_value = false
)]
pub struct EventSnapshot {
    pub id: u32,
    pub snapshot_at: DateTime<Utc>,
    pub calendar_id: u32,
    pub summary: String,
    pub location: String,
    pub description: Option<String>,
    pub start_date_time: DateTime<Utc>,
    pub end_date_time: DateTime<Utc>,
}
