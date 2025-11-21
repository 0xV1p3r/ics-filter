use chrono::{DateTime, Utc};
use diesel::prelude::*;

use super::Color;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::calendars, check_for_backend(diesel::sqlite::Sqlite))]
pub struct Calendar {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub color: Color,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
