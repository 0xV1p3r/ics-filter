use diesel::{
    Queryable,
    deserialize::FromSql,
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::{Sqlite, SqliteValue},
};
use std::fmt;

#[derive(Debug)]
pub enum FilterCriteriaType {
    Summary,
    Location,
    Description,
}

#[derive(Debug)]
pub enum FilterType {
    Blacklist,
    Whitelist,
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

impl Queryable<Text, Sqlite> for FilterCriteriaType {
    type Row = String;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(row.as_str().try_into()?)
    }
}

impl fmt::Display for FilterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            FilterType::Blacklist => "blacklist",
            FilterType::Whitelist => "whitelist",
        };
        write!(f, "{name}")
    }
}

impl TryFrom<&str> for FilterType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "blacklist" => Ok(FilterType::Blacklist),
            "whitelist" => Ok(FilterType::Whitelist),
            _ => Err(format!("Unknown filter criteria: {value}")),
        }
    }
}

impl FromSql<Text, Sqlite> for FilterType {
    fn from_sql(bytes: SqliteValue) -> diesel::deserialize::Result<Self> {
        let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        Ok(t.as_str().try_into()?)
    }
}

impl ToSql<Text, Sqlite> for FilterType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(diesel::serialize::IsNull::No)
    }
}

impl Queryable<Text, Sqlite> for FilterType {
    type Row = String;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(row.as_str().try_into()?)
    }
}
