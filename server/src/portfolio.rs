use crate::{
    database::DATABASE_CONNECTION,
    load_config::{self, Config},
};
use binance_spot_connector_rust::{http::Credentials, ureq::BinanceHttpClient, wallet};
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct BalanceSnapshotItem {
    asset: String,
    free: String,
    locked: String,
}

#[derive(Serialize, Deserialize)]
pub struct BalanceSnapshot {
    balances: Vec<BalanceSnapshotItem>,
    #[serde(rename = "totalAssetOfBtc")]
    total_asset_of_btc: String,
}

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    data: BalanceSnapshot,
    #[serde(rename = "updateTime")]
    update_time: i64,
    #[serde(rename = "type")]
    wallet_type: String,
}

#[derive(Serialize, Deserialize)]
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
    let db_query = insert_new_data(&account_history);

    db_query
}

fn insert_new_data(account_history: &AccountHistory) -> Result<()> {
    let conn = DATABASE_CONNECTION.lock().unwrap();

    // Insert account_history data
    conn.execute(
        "INSERT INTO account_history (code, msg) VALUES (?1, ?2)",
        (&account_history.code, &account_history.msg),
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

    // Commit transaction
    Ok(())
}
