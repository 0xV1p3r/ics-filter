use anyhow::{Context, Result};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .with_context(|| "Failed to load environment variable DATABASE_URL")?;
    SqliteConnection::establish(&database_url)
        .with_context(|| "Failed to establish database connection")
}
