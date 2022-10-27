use actix_web::{HttpRequest, HttpResponse};
use http::StatusCode;
use serde::Serialize;

use crate::{utils::{create_response_builder, method_logger::MethodLogger}, app_structs::AppData};

mod helper;

#[derive(Serialize)]
struct ResponseStruct {
    pub targets: Vec<helper::Target>
}

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


    let _session = match app_data.get_session(&req) {
        Some(r) => r,
        None => return respond(StatusCode::UNAUTHORIZED).finish(),
    };


    let targets = match helper::fetch_targets(app_data.mysql.clone()) {
        Some(r) => r,
        None => return respond(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    };

    let resp = ResponseStruct {
        targets
    };


    respond(StatusCode::OK).body(
        serde_json::to_string(&resp).unwrap()
    )
}