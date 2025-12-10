use diesel::prelude::*;

use super::{super::status::Status, Color};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::calendar)]
pub struct Calendar {
    pub id: u32,
    pub color: Color,
    pub name: String,
    pub status: Option<Status>,
}
