use actix_web::{HttpRequest, web::Path, HttpResponse};
use http::StatusCode;
use serde::Serialize;

use crate::{utils::{method_logger::MethodLogger, create_response_builder}, app_structs::AppData};

mod helper;

#[derive(Serialize)]
struct ResponseStruct {
    directories: Vec<String>
}

pub async fn execute(req: HttpRequest, target_path: Path<(String,)> ) -> HttpResponse {
    let logger = MethodLogger::begin(&req);
    let respond = |sc| {
        logger.finish(sc);
        create_response_builder(sc)
    };

    let app_data: &AppData = match req.app_data() {
        Some(r) => r,
        None => return respond(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    };

    let session = match app_data.get_session(&req) {
        Some(r) => r,
        None => return respond(StatusCode::UNAUTHORIZED).finish(),
    };

    if session.admin == false {
        return respond(StatusCode::UNAUTHORIZED).finish();
    }

    let targetdir = match std::fs::read_dir(&target_path.0) {
        Ok(r) => r,
        Err(_) => {
            return respond(StatusCode::BAD_REQUEST).finish();
        }
    };

    let mut directories: Vec<String> = Vec::new();

    for entry_res in targetdir {
        if entry_res.is_err() {
            continue;
        }

        let entry = entry_res.unwrap();

        if entry.path().is_dir() {
            let file_name = match entry.file_name().into_string() {
                Ok(r) => r,
                Err(_) => {
                    continue;
                }
            };

            directories.push(file_name);
        }
    }

    let resp = ResponseStruct {
        directories
    };


    respond(StatusCode::OK).body({
        serde_json::to_string(&resp).unwrap()
    })
}

/*
pub async fn execute(_mysql: Arc<RwLock<PooledConn>>, remote: Option<SocketAddr>, session_map: Arc<RwLock<HashMap<String,SessionData>>>, session: Option<String>, target_path: String) -> warp::http::Response<String> {
    let start_time = Instant::now();
    let info_mesg = |sc: StatusCode| {
        print_conneciton_info(remote, METHOD, ROUTE, sc, start_time.elapsed())
    };



    let generate_error_response = |responsecode: StatusCode| {
        info_mesg(responsecode);
        utils::create_response_builder().status(responsecode).body(
            String::from("")
        ).unwrap()
    };

    let target_path_decoded = match urlencoding::decode(&target_path) {
        Ok(r) => r.to_string(),
        Err(_) => {
            return generate_error_response(StatusCode::BAD_REQUEST);
        }
    };

    let sesion_key = match session {
        Some(r) => r,
        None => {
            return generate_error_response(StatusCode::UNAUTHORIZED);
        }
    };

    let is_admin;

    match session_map.read() {
        Ok(r) => {
            match r.get(&sesion_key) {
                Some(r) => {
                    is_admin = r.admin;
                },
                None => {
                    return generate_error_response(StatusCode::UNAUTHORIZED);
                }
            }
        },
        Err(_) => {
            return generate_error_response(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if is_admin == false {
        return generate_error_response(StatusCode::UNAUTHORIZED);
    }

    let targetdir = match std::fs::read_dir(target_path_decoded) {
        Ok(r) => r,
        Err(_) => {
            return generate_error_response(StatusCode::BAD_REQUEST);
        }
    };

    let mut directories: Vec<String> = Vec::new();

    for entry_res in targetdir {
        if entry_res.is_err() {
            continue;
        }

        let entry = entry_res.unwrap();

        if entry.path().is_dir() {
            let file_name = match entry.file_name().into_string() {
                Ok(r) => r,
                Err(_) => {
                    continue;
                }
            };

            directories.push(file_name);
        }
    }

    let resp = ResponseStruct {
        directories
    };

    info_mesg(StatusCode::OK);
    
    utils::create_response_builder().body(
        serde_json::to_string(&resp).unwrap()
    ).unwrap()
}*/