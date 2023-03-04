mod cli;
mod database;
mod load_config;
mod plot;
mod portfolio;

use chrono::prelude::*;
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
    let mut date_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut last_event_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut last_event = String::from("Startup");
    cli::render(&date_time, &last_event, &last_event_time);

    // Get portfolio state
    last_event_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    last_event = String::from("Startup");
    cli::render(&date_time, &last_event, &last_event_time);

    loop {
        date_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        cli::render(&date_time, &last_event, &last_event_time);
        // Wait for 5 seconds before next query
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
    println!("Table setup complete");
    portfolio::fetch_account_balance_history().await.unwrap();
    println!("Fetched account status");
    // Spawn a new task to query data source concurrently with Rocket server
    tokio::spawn(start_cli());

    rocket::build()
        .mount("/", routes![plot::get_plot])
        .mount("/", FileServer::new("static", Options::None).rank(1))
        .register("/", catchers![internal_error, not_found, default])
}
