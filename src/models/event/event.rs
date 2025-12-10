use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::super::{calendar::Calendar, status::Status};

#[derive(Queryable, Selectable, Identifiable, Associations, Deserialize, Serialize)]
#[diesel(
    belongs_to(Calendar),
    table_name = crate::schema::event,
    treat_none_as_default_value = false
)]
pub struct Event {
    pub id: u32,
    pub calendar_id: u32,
    pub summary: String,
    pub location: String,
    pub description: Option<String>,
    pub start_date_time: DateTime<Utc>,
    pub end_date_time: DateTime<Utc>,
    pub status: Option<Status>,
}
