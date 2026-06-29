use config::File;
use rusqlite::{Connection, Result};
use serde::Deserialize;
use std::{thread, time::Duration};
mod constants;

#[derive(Debug, Deserialize)]
struct Config {
    database: DBConfig,
}

#[derive(Debug, Deserialize)]
struct DBConfig {
    name: String,
}

struct NewMessage {
    timestamp: i64,
    content: String,
    timestamp_sent: i64,
}

fn main() -> Result<()> {
    let conn = connect_db()?;

    let mut string_counter: i32 = 0;

    loop {
        let current_counter: i32 = conn
            .prepare("SELECT COUNT(*) FROM timer;")?
            .query_row([], |row| row.get(0))?;
        println!("записей в БД: {current_counter}");

        while current_counter > string_counter {
            let mut stmt = conn.prepare(
                "SELECT timestamp, char_string, timestamp_sent FROM timer LIMIT 1 OFFSET ?",
            )?;
            let row: NewMessage = stmt.query_row([string_counter], |row| {
                Ok(NewMessage {
                    timestamp: row.get(0)?,
                    content: row.get(1)?,
                    timestamp_sent: row.get(2)?,
                })
            })?;
            println!(
                "Timestamp: {}, Content: {}, Timestamp Sent: {}",
                row.timestamp, row.content, row.timestamp_sent
            );
            string_counter += 1;
        }

        thread::sleep(Duration::from_millis(constants::SLEEP_DURATION_THREAD));
    }
}

fn connect_db() -> Result<Connection> {
    let config: Config = config::Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("Failed to build config from config.toml")
        .try_deserialize()
        .expect("Failed to deserialize configuration");

    let db_path = config.database.name;

    let connect = Connection::open(&db_path)?;

    Ok(connect)
}
