use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    pub static ref DATABASE_CONNECTION: Arc<Mutex<Connection>> =
        Arc::new(Mutex::new(Connection::open("database.sqlite").unwrap()));
}

pub fn get_database_connection() -> rusqlite::Connection {
    Connection::open("database.sqlite").unwrap()
}

pub fn setup_tables() -> Result<()> {
    let connection = DATABASE_CONNECTION.lock().unwrap();

    connection
        .execute_batch(
            "BEGIN;
        CREATE TABLE IF NOT EXISTS balances (
            id INTEGER PRIMARY KEY,
            asset TEXT NOT NULL,
            free TEXT NOT NULL,
            locked TEXT NOT NULL,
            snapshot_id INTEGER NOT NULL,
            FOREIGN KEY(snapshot_id) REFERENCES snapshots(id)
        );
        CREATE TABLE IF NOT EXISTS snapshots (
            id INTEGER PRIMARY KEY,
            total_asset_of_btc TEXT NOT NULL,
            update_time INTEGER NOT NULL,
            wallet_type TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS account_history (
            id INTEGER PRIMARY KEY,
            code INTEGER NOT NULL,
            msg TEXT NOT NULL,
            last_queried DATETIME NOT NULL
        );
        COMMIT;",
        )
        .map_err(|error| {
            println!("Init database error: {:?}", error);
            error
        })
}
