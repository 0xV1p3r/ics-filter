use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    serialize::{IsNull, Output, ToSql},
    sql_types::SqlType,
};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(SqlType)]
#[diesel(postgres_type(name = "status_enum"))]
struct StatusSqlType;

#[derive(Debug, FromSqlRow, AsExpression, Deserialize, Serialize)]
#[diesel(sql_type = StatusSqlType)]
pub enum Status {
    Archived,
    Deleted,
    Trash,
}

impl Into<&[u8]> for &Status {
    fn into(self) -> &'static [u8] {
        match self {
            Status::Archived => b"archived",
            Status::Deleted => b"deleted",
            Status::Trash => b"Trash",
        }
    }
}

impl TryFrom<&[u8]> for Status {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"archived" => Ok(Status::Archived),
            b"deleted" => Ok(Status::Deleted),
            b"trash" => Ok(Status::Trash),
            _ => Err("Unknown enum variant"),
        }
    }
}

impl FromSql<StatusSqlType, Pg> for Status {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        Ok(bytes.as_bytes().try_into()?)
    }
}

impl ToSql<StatusSqlType, Pg> for Status {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        out.write_all(self.into())?;
        Ok(IsNull::No)
    }
}
