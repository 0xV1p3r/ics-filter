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
#[diesel(table_name = crate::schema::filter_criteria)]
pub struct FilterCriteria {
    pub id: u32,
    pub filter_criteria_type: FilterCriteriaType,
    pub value: String,
}

#[derive(SqlType)]
#[diesel(postgres_type(name = "filter_criteria_enum"))]
struct FilterCriteriaSqlType;

#[derive(Debug, FromSqlRow, AsExpression)]
#[diesel(sql_type = FilterCriteriaSqlType)]
pub enum FilterCriteriaType {
    Summary,
    Location,
    Description,
}

impl Into<&[u8]> for &FilterCriteriaType {
    fn into(self) -> &'static [u8] {
        match self {
            FilterCriteriaType::Description => b"description",
            FilterCriteriaType::Location => b"location",
            FilterCriteriaType::Summary => b"summary",
        }
    }
}

impl TryFrom<&[u8]> for FilterCriteriaType {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"description" => Ok(FilterCriteriaType::Description),
            b"location" => Ok(FilterCriteriaType::Location),
            b"summary" => Ok(FilterCriteriaType::Summary),
            _ => Err("Unknown enum variant"),
        }
    }
}

impl FromSql<FilterCriteriaSqlType, Pg> for FilterCriteriaType {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        Ok(bytes.as_bytes().try_into()?)
    }
}

impl ToSql<FilterCriteriaSqlType, Pg> for FilterCriteriaType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        out.write_all(self.into())?;
        Ok(IsNull::No)
    }
}
