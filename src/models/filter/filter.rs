use chrono::{DateTime, Utc};
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::*,
    serialize::{IsNull, Output, ToSql},
    sql_types::SqlType,
};
use std::io::Write;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::filters)]
pub struct Filter {
    pub id: i32,
    pub filter_type: FilterType,
    pub name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(SqlType)]
#[diesel(postgres_type(name = "filter_type_enum"))]
struct FilterSqlType;

#[derive(Debug, FromSqlRow, AsExpression)]
#[diesel(sql_type = FilterSqlType)]
pub enum FilterType {
    Blacklist,
    Whitelist,
}

impl Into<&[u8]> for &FilterType {
    fn into(self) -> &'static [u8] {
        match self {
            FilterType::Blacklist => b"blacklist",
            FilterType::Whitelist => b"whitelist",
        }
    }
}

impl TryFrom<&[u8]> for FilterType {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"blacklist" => Ok(FilterType::Blacklist),
            b"whitelist" => Ok(FilterType::Whitelist),
            _ => Err("Unknown enum variant"),
        }
    }
}

impl FromSql<FilterSqlType, Pg> for FilterType {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        Ok(bytes.as_bytes().try_into()?)
    }
}

impl ToSql<FilterSqlType, Pg> for FilterType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        out.write_all(self.into())?;
        Ok(IsNull::No)
    }
}
