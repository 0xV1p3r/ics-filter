use diesel::{
    Queryable,
    deserialize::FromSql,
    serialize::{Output, ToSql},
    sql_types::Text,
    sqlite::{Sqlite, SqliteValue},
};
use std::fmt;

#[derive(Debug)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Gray,
    Orange,
    Purple,
    Brown,
    Pink,
}

impl Color {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Black => (0, 0, 0),
            Color::White => (255, 255, 255),
            Color::Red => (255, 0, 0),
            Color::Green => (0, 128, 0),
            Color::Blue => (0, 0, 255),
            Color::Yellow => (255, 255, 0),
            Color::Cyan => (0, 255, 255),
            Color::Magenta => (255, 0, 255),
            Color::Gray => (128, 128, 128),
            Color::Orange => (255, 165, 0),
            Color::Purple => (128, 0, 128),
            Color::Brown => (165, 42, 42),
            Color::Pink => (255, 192, 203),
        }
    }

    pub fn to_hex(&self) -> &'static str {
        match self {
            Color::Black => "#000000",
            Color::White => "#FFFFFF",
            Color::Red => "#FF0000",
            Color::Green => "#008000",
            Color::Blue => "#0000FF",
            Color::Yellow => "#FFFF00",
            Color::Cyan => "#00FFFF",
            Color::Magenta => "#FF00FF",
            Color::Gray => "#808080",
            Color::Orange => "#FFA500",
            Color::Purple => "#800080",
            Color::Brown => "#A52A2A",
            Color::Pink => "#FFC0CB",
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Color::Black => "Black",
            Color::White => "White",
            Color::Red => "Red",
            Color::Green => "Green",
            Color::Blue => "Blue",
            Color::Yellow => "Yellow",
            Color::Cyan => "Cyan",
            Color::Magenta => "Magenta",
            Color::Gray => "Gray",
            Color::Orange => "Orange",
            Color::Purple => "Purple",
            Color::Brown => "Brown",
            Color::Pink => "Pink",
        };
        write!(f, "{name}")
    }
}

impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_ascii_lowercase().as_str() {
            "black" => Ok(Color::Black),
            "white" => Ok(Color::White),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            "yellow" => Ok(Color::Yellow),
            "cyan" => Ok(Color::Cyan),
            "magenta" => Ok(Color::Magenta),
            "gray" | "grey" => Ok(Color::Gray),
            "orange" => Ok(Color::Orange),
            "purple" => Ok(Color::Purple),
            "brown" => Ok(Color::Brown),
            "pink" => Ok(Color::Pink),
            _ => Err(format!("Unknown color: {value}")),
        }
    }
}

impl FromSql<Text, Sqlite> for Color {
    fn from_sql(bytes: SqliteValue) -> diesel::deserialize::Result<Self> {
        let t = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        Ok(t.as_str().try_into()?)
    }
}

impl ToSql<Text, Sqlite> for Color {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(diesel::serialize::IsNull::No)
    }
}

impl Queryable<Text, Sqlite> for Color {
    type Row = String;

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(row.as_str().try_into()?)
    }
}
