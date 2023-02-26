use crate::load_config::{self, Config};
use binance_spot_connector_rust::{http::Credentials, ureq::BinanceHttpClient, wallet};
use rocket::get;
use std::time::{SystemTime, UNIX_EPOCH};

#[get("/portfolio")]
pub async fn get_portfolio() -> String {
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
    let data = client
        .send(wallet::account_snapshot("SPOT").start_time((timestamp - 3600 * 24 * 30) * 1000))
        .expect("Request failed")
        .into_body_str()
        .expect("Failed to read response body");

    println!("data {}", data);
    data
}
