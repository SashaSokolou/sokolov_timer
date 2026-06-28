use rusqlite::{Connection, Result};
use std::{thread, time::Duration};

struct NewMessage {
    timestamp: i64,
    content: String,
    timestamp_sent: i64,
}

fn main() -> Result<()> {
    let conn = Connection::open("timer_db.db")?;

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

        thread::sleep(Duration::from_secs(5));
    }
}
