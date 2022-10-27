use actix_files::NamedFile;
use actix_web::{HttpRequest, web::Path, Responder};
use http::StatusCode;

use crate::{utils::{method_logger::MethodLogger, create_response_builder}, app_structs::AppData};

mod helper;

pub async fn execute(req: HttpRequest, target_data: Path<(String, String)> ) -> impl Responder {
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

    let root_path = match helper::get_root_path(app_data.mysql.clone(), &target_data.0) {
        Ok(r) => r,
        Err(e) => return respond(e).finish(),
    };

    let full_path = match helper::assemble_full_path(&root_path, &target_data.1) {
        Ok(r) => r,
        Err(e) => return respond(e).finish(),
    };

    logger.finish(StatusCode::OK);
    NamedFile::open_async(full_path)
        .await.unwrap().into_response(&req)
}