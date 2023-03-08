use crate::{
    database::DATABASE_CONNECTION,
    load_config::{self, Config},
};
use binance_spot_connector_rust::{http::Credentials, ureq::BinanceHttpClient, wallet};
use chrono::{prelude::*, Duration};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceSnapshotItem {
    asset: String,
    free: String,
    locked: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceSnapshot {
    balances: Vec<BalanceSnapshotItem>,
    #[serde(rename = "totalAssetOfBtc")]
    total_asset_of_btc: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
    data: BalanceSnapshot,
    #[serde(rename = "updateTime")]
    update_time: i64,
    #[serde(rename = "type")]
    wallet_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountHistory {
    code: i32,
    msg: String,
    #[serde(rename = "snapshotVos")]
    snapshot_vos: Vec<Snapshot>,
}

pub async fn fetch_account_balance_history() -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let config: Config = load_config::read_config();
    let credentials = Credentials::from_hmac(
        config.binance_api_key.to_owned(),
        config.binance_api_secret.to_owned(),
    );
    let client = BinanceHttpClient::default().credentials(credentials);

    // Get account information
    let response = client
        .send(wallet::account_snapshot("SPOT").start_time((timestamp - (3600 * 24 * 30)) * 1000))
        .expect("Request failed")
        .into_body_str()
        .expect("Failed to read response body");

    let account_history: AccountHistory = serde_json::from_str(&response).unwrap();
    let db_query = insert_account_history(&account_history);

    db_query
}

fn insert_account_history(account_history: &AccountHistory) -> Result<()> {
    let conn = DATABASE_CONNECTION.lock().unwrap();
    let timestamp: DateTime<Utc> = Utc::now() - Duration::days(1);

    let mut stmt =
        conn.prepare("SELECT id FROM account_history WHERE last_queried > ?1 LIMIT 1")?;
    let has_last_queried_today = stmt.exists(params![timestamp])?;

    if !has_last_queried_today {
        conn.execute(
            "DELETE FROM account_history; DELETE FROM snapshot; DELETE FROM balances;",
            (),
        )?;
        // Insert account_history data
        conn.execute(
            "INSERT INTO account_history (code, msg, last_queried) VALUES (?1, ?2, ?3)",
            (&account_history.code, &account_history.msg, Utc::now()),
        )?;

        // Insert snapshot data
        for snapshot in &account_history.snapshot_vos {
            let snapshot_id = conn.execute(
                "INSERT INTO snapshots (total_asset_of_btc, update_time, wallet_type) VALUES (?1, ?2, ?3)",
                (
                    &snapshot.data.total_asset_of_btc,
                    &snapshot.update_time,
                    &snapshot.wallet_type,
            ))?;
            // Insert balances data
            let balances = &snapshot.data.balances;
            for balance in balances {
                conn.execute(
                    "INSERT INTO balances (asset, free, locked, snapshot_id) VALUES (?1, ?2, ?3, ?4)",
                    (&balance.asset, &balance.free, &balance.locked, &snapshot_id),
            )?;
            }
        }
    }

    // Commit transaction
    Ok(())
}

pub fn get_account_history_with_snapshots(conn: &Connection) -> Result<Option<AccountHistory>> {
    let mut stmt = conn.prepare(
        "SELECT account_history.*, snapshots.id as snapshot_id, snapshots.total_asset_of_btc, snapshots.update_time, snapshots.wallet_type, balances.id as balances_id, balances.asset, balances.free, balances.locked 
        FROM account_history 
        INNER JOIN snapshots ON account_history.id = snapshots.id 
        INNER JOIN balances ON snapshots.id = balances.snapshot_id 
		WHERE free IS NOT 0",
    )?;

    // id, code, msg, last_queried, snapshot_id, total_asset_of_btc, update_time, wallet_type, balances_id, asset, free,
    // locked

    let rows = stmt.query_map([], |row| {
        let mut snapshot = Snapshot {
            data: BalanceSnapshot {
                balances: Vec::new(),
                total_asset_of_btc: row.get(5)?,
            },
            update_time: row.get(6)?,
            wallet_type: row.get(7)?,
        };
        snapshot.data.balances.push(BalanceSnapshotItem {
            asset: row.get(9)?,
            free: row.get(10)?,
            locked: row.get(11)?,
        });

        Ok((snapshot, row.get(1)?, row.get(2)?))
    })?;

    let mut account_history = None;

    for row in rows {
        let (snapshot, code, msg) = row?;

        if account_history.is_none() {
            account_history = Some(AccountHistory {
                code,
                msg,
                snapshot_vos: vec![snapshot],
            });
        } else {
            account_history
                .as_mut()
                .unwrap()
                .snapshot_vos
                .push(snapshot);
        }
    }

    Ok(account_history)
}
