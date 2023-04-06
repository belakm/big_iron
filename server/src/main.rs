// mod api;
mod book;
mod database;
// mod formatting;
// mod plot;
// mod prediction_model;

use rocket::catch;
use rocket::fs::FileServer;
use rocket::fs::Options;
use rocket::http::Status;
use rocket::Request;

#[macro_use]
extern crate rocket;

#[catch(500)]
fn internal_error() -> &'static str {
    "Error 500; something is not clicking right."
}

#[catch(404)]
fn not_found() -> &'static str {
    "Error 404; nothing here fren."
}

#[catch(default)]
fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}

#[rocket::main]
async fn main() -> Result<(), String> {
    match database::initialize().await {
        Ok(_) => {
            tokio::spawn(book::main());
            let rocket_startup =
                rocket::build() // .mount("/", routes![api::account_balance_history])
                    .mount("/", FileServer::new("static", Options::None).rank(1))
                    .register("/", catchers![internal_error, not_found, default])
                    .launch()
                    .await;
            match rocket_startup {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
