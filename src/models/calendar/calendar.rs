use diesel::prelude::*;

use super::Color;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::calendar)]
pub struct Calendar {
    pub id: u32,
    pub name: String,
    pub color: Color,
}
