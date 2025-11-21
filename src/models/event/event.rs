use chrono::{DateTime, Utc};
use diesel::prelude::*;

use super::super::calendar::Calendar;

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(
    belongs_to(Calendar),
    table_name = crate::schema::events,
    treat_none_as_default_value = false
)]
pub struct Event {
    pub id: i32,
    pub calendar_id: i32,
    pub uid: String,
    pub summary: String,
    pub location: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    created_at: DateTime<Utc>,
}
