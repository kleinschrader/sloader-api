use actix_web::{HttpRequest, HttpResponse};
use http::StatusCode;
use serde::Serialize;

use crate::{app_structs::AppData, utils::{create_response_builder, method_logger::MethodLogger}};

#[derive(Serialize)]
struct ResponseStruct {
    authenticated: bool
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

    let session = app_data.get_session(&req);

    let resp = ResponseStruct {
        authenticated: session.is_some()
    };

    let mut builder = respond(StatusCode::OK);

    builder.body(
        serde_json::to_string(&resp).unwrap()
    )
}
