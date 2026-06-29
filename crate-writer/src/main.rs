use config::File;
use rand::distr::{Alphanumeric, SampleString};
use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{sync::mpsc, thread, time::Duration};
mod constants;

#[derive(Debug, Deserialize)]
struct Config {
    database: DBConfig,
}

#[derive(Debug, Deserialize)]
struct DBConfig {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CustomMessage {
    timestamp: i64,
    content: String,
    timestamp_sent: i64,
}

fn main() -> Result<()> {
    let conn = connect_db()?;
    let _ = create_table(&conn);

    let (tx, rx) = mpsc::channel::<String>();

    let thread2 = thread::spawn(move || {
        let mut threads_message = CustomMessage {
            timestamp: 0,
            content: String::new(),
            timestamp_sent: 0,
        };

        for _i in 1..(constants::MAX_DB_LINES_PLUS_ONE) {
            threads_message.content = gen_random_string();
            threads_message.timestamp_sent = get_current_timestamp();
            let serialized_message =
                serde_json::to_string(&threads_message).expect("Failed to serialize message");
            tx.send(serialized_message).expect("Failed to send message");

            thread::sleep(Duration::from_millis(constants::SLEEP_DURATION_THREAD));
        }
    });

    let thread1 = thread::spawn(move || {
        for _i in 1..31 {
            let received_message = rx.recv().expect("Failed to receive message");
            let mut deserialized_message: CustomMessage =
                serde_json::from_str(&received_message).expect("Failed to deserialize message");
            deserialized_message.timestamp = get_current_timestamp();
            let _ = insert_message(&conn, deserialized_message);
            //thread::sleep(Duration::from_millis(constants::SLEEP_DURATION_THREAD);
        }
    });
    thread1.join().expect("Thread1 error");
    thread2.join().expect("Thread2 error");
    Ok(())
}

fn connect_db() -> Result<Connection> {
    let config: Config = config::Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()
        .expect("Failed to build config from config.toml")
        .try_deserialize()
        .expect("Failed to deserialize config");

    let db_path = config.database.name;

    let connect = Connection::open(&db_path)?;

    Ok(connect)
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists timer (
             timestamp integer primary key,
             char_string text not null unique,
             timestamp_sent integer             
        )",
        [],
    )?;
    Ok(())
}

fn gen_random_string() -> String {
    let str = Alphanumeric.sample_string(&mut rand::rng(), constants::MAX_SIZE_DB_LINE_LENGTH);
    str.to_string()
}

fn get_current_timestamp() -> i64 {
    let tmstmp = SystemTime::now();
    let after_start_unix_epoch = tmstmp.duration_since(UNIX_EPOCH).expect("some digits");
    after_start_unix_epoch.as_millis() as i64
}

fn insert_message(conn: &Connection, message: CustomMessage) -> Result<()> {
    conn.execute(
        "INSERT INTO timer (timestamp, char_string, timestamp_sent) VALUES (?1, ?2, ?3)",
        params![message.timestamp, message.content, message.timestamp_sent],
    )?;
    Ok(())
}
