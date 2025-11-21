use chrono::{DateTime, Utc};
use diesel::prelude::*;

use super::{super::filter::Filter, Calendar, Color};

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(
    belongs_to(Calendar),
    belongs_to(Filter),
    table_name = crate::schema::filtered_calendars
)]
pub struct FilteredCalendar {
    pub id: i32,
    pub calendar_id: i32,
    pub filter_id: i32,
    pub name: String,
    pub color: Color,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
