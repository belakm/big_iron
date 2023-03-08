use rocket::http::Status;
use rocket::response::status::{Custom, NotFound};

pub static MAP_TO_500: fn(&str) -> ServerError =
    |err| ServerError::Custom(Custom(Status::InternalServerError, err.to_string()));

pub static MAP_TO_404: fn(&str) -> ServerError =
    |err| ServerError::NotFound(NotFound(err.to_string()));

#[derive(Debug, Responder)]
pub enum ServerError {
    Custom(Custom<String>),
    NotFound(NotFound<String>),
}
