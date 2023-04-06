use binance_spot_connector_rust::{
    market::klines::KlineInterval, market_stream::kline::KlineStream,
    tungstenite::BinanceWebSocketClient,
};
use std::time::Duration;
use tokio::time::sleep;

const BINANCE_WSS_BASE_URL: &str = "wss://stream.binance.com:9443/ws";

pub async fn subscribe_to_price_updates() {
    // Establish connection
    let mut conn =
        BinanceWebSocketClient::connect_with_url(BINANCE_WSS_BASE_URL).expect("Failed to connect");
    // Subscribe to streams
    conn.subscribe(vec![
        &KlineStream::new("BTCUSDT", KlineInterval::Minutes1).into(),
        &KlineStream::new("BNBBUSD", KlineInterval::Minutes3).into(),
    ]);
    // Read messages
    while let Ok(message) = conn.as_mut().read_message() {
        let data = message.into_data();
        let string_data = String::from_utf8(data).expect("Found invalid UTF-8 chars");
        println!("Socket tick.");
    }
    // Disconnect
    conn.close().expect("Failed to disconnect");

    // TODO: Update to database
}

async fn update() {
    loop {
        sleep(Duration::from_secs(10)).await;
    }
}

fn get_portfolio() {}

pub async fn main() {
    // start updating book records
    get_portfolio();
    tokio::spawn(update());
    tokio::spawn(subscribe_to_price_updates());
}
