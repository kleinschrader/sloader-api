use std::{fs::File, io::Write};

use actix_multipart::Multipart;
use actix_web::{HttpRequest, web::Path, HttpResponse};
use http::StatusCode;

use futures::StreamExt;
use log::error;

use crate::{utils::{method_logger::MethodLogger, create_response_builder}, app_structs::AppData};

mod helper;

pub async fn execute(req: HttpRequest, target_data: Path<(String, String)> , mut payload: Multipart) -> HttpResponse {
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

    let mut target_file = match File::create(full_path) {
        Ok(r) => r,
        Err(_) => return respond(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    };

    while let Some(item) = payload.next().await {
        let mut field = match item{
            Ok(r) => r,
            Err(e) => {
                error!("Error parsing multipart: {}", e);
                return respond(StatusCode::INTERNAL_SERVER_ERROR).finish()
            },
        };

        // Field in turn is stream of *Bytes* object
        while let Some(chunk_res) = field.next().await {
            let chunk = match chunk_res {
                Ok(r) => r,
                Err(_) => return respond(StatusCode::INTERNAL_SERVER_ERROR).finish()
            };


            if target_file.write(chunk.as_ref()).is_err() {
                return respond(StatusCode::INTERNAL_SERVER_ERROR).finish();
            }
        }
    }  

    respond(StatusCode::OK).finish()
}

