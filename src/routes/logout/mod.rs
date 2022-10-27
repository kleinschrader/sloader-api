use actix_web::{HttpRequest, HttpResponse};
use http::StatusCode;

use crate::{utils::{method_logger::MethodLogger, create_response_builder}, app_structs::AppData};



pub async fn execute(req: HttpRequest) -> HttpResponse {
    let logger = MethodLogger::begin(&req);
    let respond = |sc| {
        logger.finish(sc);
        create_response_builder(sc)
    };

    let app_data: &AppData = match req.app_data() {
        Some(r) => r,
        None => return respond(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    };


    let mut sm = match app_data.session_map.write() {
        Ok(r) => r,
        Err(_) => return respond(StatusCode::OK).finish(),
    };

    let cookie_session = match req.cookie("SESSION") {
        Some(r) => r,
        None => return respond(StatusCode::OK).finish(),
    };

    sm.remove_entry(cookie_session.value());

    respond(StatusCode::OK).finish()
}