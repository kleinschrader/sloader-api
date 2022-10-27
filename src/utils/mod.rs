use std::env;

use actix_web::HttpResponseBuilder;
use http::StatusCode;

pub mod session;
pub mod mysql;
pub mod method_logger;

pub fn create_response_builder(status: StatusCode) -> HttpResponseBuilder {
    let cors_allow: String = env::var("CORS_ALLOW_ORIGIN").unwrap();


    let mut r = HttpResponseBuilder::new(status);
        
    r.insert_header(("Access-Control-Allow-Origin", cors_allow))
        .insert_header(("Access-Control-Allow-Credentials", "true"))
        .insert_header(("Content-Type", "application/json"));

    r
}