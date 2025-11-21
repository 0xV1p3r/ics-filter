use diesel::{
    deserialize::FromSql,
    pg::{Pg, PgValue},
    serialize::{IsNull, Output, ToSql},
    sql_types::Text,
};
use std::io::Write;

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

impl Into<&[u8]> for &Color {
    fn into(self) -> &'static [u8] {
        match self {
            Color::Black => b"black",
            Color::White => b"white",
            Color::Red => b"red",
            Color::Green => b"green",
            Color::Blue => b"blue",
            Color::Yellow => b"yellow",
            Color::Cyan => b"cyan",
            Color::Magenta => b"magenta",
            Color::Gray => b"gray",
            Color::Orange => b"orange",
            Color::Purple => b"purple",
            Color::Brown => b"brown",
            Color::Pink => b"pink",
        }
    }
}

impl TryFrom<&[u8]> for Color {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"black" => Ok(Color::Black),
            b"white" => Ok(Color::White),
            b"red" => Ok(Color::Red),
            b"green" => Ok(Color::Green),
            b"blue" => Ok(Color::Blue),
            b"yellow" => Ok(Color::Yellow),
            b"cyan" => Ok(Color::Cyan),
            b"magenta" => Ok(Color::Magenta),
            b"gray" | b"grey" => Ok(Color::Gray),
            b"orange" => Ok(Color::Orange),
            b"purple" => Ok(Color::Purple),
            b"brown" => Ok(Color::Brown),
            b"pink" => Ok(Color::Pink),
            _ => Err("Unknown enum variant"),
        }
    }
}

impl FromSql<Text, Pg> for Color {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        Ok(bytes.as_bytes().try_into()?)
    }
}

impl ToSql<Text, Pg> for Color {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        out.write_all(self.into())?;
        Ok(IsNull::No)
    }
}
