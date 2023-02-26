#[macro_use]
extern crate rocket;

mod load_config;
mod plot;
mod portfolio;

use rocket::catch;
use rocket::fs::Options;
use rocket::http::Status;
use rocket::Request;
use rocket::{fs::FileServer, launch, routes};

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
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![plot::get_plot, portfolio::get_portfolio])
        .mount("/", FileServer::new("static", Options::None).rank(1))
        .register("/", catchers![internal_error, not_found, default])
}
