use rand::distr::{Alphanumeric, SampleString};
use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{sync::mpsc, thread, time::Duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CustomMessage {
    timestamp: i64,
    content: String,
    timestamp_sent: i64,
}

fn main() {
    let conn = connect_db().unwrap();
    let _ = create_table(&conn);

    let (tx, rx) = mpsc::channel::<String>();

    let thread2 = thread::spawn(move || {
        let mut threads_message = CustomMessage {
            timestamp: 0,
            content: String::new(),
            timestamp_sent: 0,
        };

        for _i in 1..31 {
            threads_message.content = gen_random_string();
            threads_message.timestamp_sent = get_current_timestamp();
            let serialized_message = serde_json::to_string(&threads_message).unwrap();
            tx.send(serialized_message).unwrap();

            thread::sleep(Duration::from_secs(60));
        }
    });

    let thread1 = thread::spawn(move || {
        for _i in 1..31 {
            let received_message = rx.recv().unwrap();
            let mut deserialized_message: CustomMessage =
                serde_json::from_str(&received_message).unwrap();
            deserialized_message.timestamp = get_current_timestamp();
            let _ = insert_message(&conn, deserialized_message);
            //thread::sleep(Duration::from_secs(2));
        }
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn connect_db() -> Result<Connection> {
    let path = "./timer_db.db";
    let db: Connection = Connection::open(path)?;

    Ok(db)
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
    let str = Alphanumeric.sample_string(&mut rand::rng(), 255);
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
