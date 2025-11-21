use chrono::{DateTime, Utc};
use diesel::{
    deserialize::FromSql,
    prelude::*,
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::{Sqlite, SqliteValue},
};
use std::fmt;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::filter_criteria, check_for_backend(Sqlite))]
pub struct FilterCriteria {
    pub id: i32,
    pub criteria_type: FilterCriteriaType,
    pub value: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum FilterCriteriaType {
    Summary,
    Location,
    Description,
}

impl fmt::Display for FilterCriteriaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            FilterCriteriaType::Description => "description",
            FilterCriteriaType::Location => "location",
            FilterCriteriaType::Summary => "summary",
        };
        write!(f, "{name}")
    }
}

impl FromSql<Text, Sqlite> for FilterCriteriaType {
    fn from_sql(bytes: SqliteValue) -> diesel::deserialize::Result<Self> {
        let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        Ok(t.as_str().try_into()?)
    }
}

impl ToSql<Text, Sqlite> for FilterCriteriaType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(diesel::serialize::IsNull::No)
    }
}

impl TryFrom<&str> for FilterCriteriaType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "description" => Ok(FilterCriteriaType::Description),
            "location" => Ok(FilterCriteriaType::Location),
            "summary" => Ok(FilterCriteriaType::Summary),
            _ => Err(format!("Unknown filter criteria: {value}")),
        }
    }
}

impl Queryable<Text, Sqlite> for FilterCriteriaType {
    type Row = String;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(row.as_str().try_into()?)
    }
}
