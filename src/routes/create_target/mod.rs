use actix_web::{HttpRequest, HttpResponse};
use bytes::Bytes;
use http::StatusCode;
use serde::Deserialize;
use uuid::Uuid;

use crate::{utils::{method_logger::MethodLogger, create_response_builder}, app_structs::AppData};

use self::helper::insert_target;

mod helper;

#[derive(Deserialize)]
struct RequestStruct {
    name: String,
    path: String,
}

pub async fn execute(req: HttpRequest, body: Bytes) -> HttpResponse {

    let logger = MethodLogger::begin(&req);
    let respond = |sc| {
        logger.finish(sc);
        create_response_builder(sc)
    };

    let app_data: &AppData = match req.app_data() {
        Some(r) => r,
        None => return respond(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    };

    let request_data: RequestStruct = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(_) => return respond(StatusCode::BAD_REQUEST).finish(),
    };

    let session = match app_data.get_session(&req) {
        Some(r) => r,
        None => return respond(StatusCode::UNAUTHORIZED).finish(),
    };

    if session.admin == false {
        return respond(StatusCode::UNAUTHORIZED).finish();
    }

    if request_data.name.len() > 128 {
        return respond(StatusCode::BAD_REQUEST).finish();
    }

    if request_data.path.len() > 4096 {
        return respond(StatusCode::BAD_REQUEST).finish();
    }

    let generated_uuid = Uuid::new_v4();

    match insert_target(app_data.mysql.clone(), &request_data.name, &request_data.path, generated_uuid) {
        Ok(_) => respond(StatusCode::OK).finish(),
        Err(_) => respond(StatusCode::INTERNAL_SERVER_ERROR).finish()
    }
}