use diesel::prelude::*;

use super::{super::filter::Filter, Calendar};

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(
    belongs_to(Calendar, foreign_key = id),
    belongs_to(Filter),
    table_name = crate::schema::filtered_calendar
)]
pub struct FilteredCalendar {
    pub id: u32,
    pub source_id: u32,
    pub filter_id: u32,
}
