mod cli;
mod database;
mod formatting;
mod load_config;
mod plot;
mod portfolio;

use rocket::catch;
use rocket::fs::Options;
use rocket::http::Status;
use rocket::Request;
use rocket::{fs::FileServer, launch, routes};

use std::time::Duration;
use tokio::time::sleep;

#[macro_use]
extern crate rocket;

async fn start_cli() {
    // Startup
    cli::render("Starting loop.");
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}

#[launch]
async fn rocket() -> _ {
    // Init DB
    database::setup_tables().unwrap();
    cli::render("SQLite: table setup complete");
    portfolio::fetch_account_balance_history().await.unwrap();
    cli::render("Binance: Account status fetched");
    // Spawn a new task to query data source concurrently with Rocket server
    tokio::spawn(start_cli());
    rocket::build()
        .mount("/", routes![plot::get_plot])
        .mount("/", FileServer::new("static", Options::None).rank(1))
        .register("/", catchers![internal_error, not_found, default])
}
