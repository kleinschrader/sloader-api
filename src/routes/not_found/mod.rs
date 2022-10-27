use actix_web::{HttpRequest, HttpResponse};
use http::StatusCode;

use crate::utils::{method_logger::MethodLogger, create_response_builder};

pub async fn execute(req: HttpRequest) -> HttpResponse {
    let logger = MethodLogger::begin(&req);

    logger.finish(StatusCode::NOT_FOUND);

    create_response_builder(StatusCode::NOT_FOUND).finish()
}