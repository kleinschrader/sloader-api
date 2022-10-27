

use actix_web::{HttpRequest, HttpResponse, web::Path};
use http::StatusCode;
use serde::Serialize;

use crate::{utils::{method_logger::MethodLogger, create_response_builder}, app_structs::AppData};

mod helper;

#[derive(Serialize)]
struct ResponseFile {
    name: String,
    size: u64,
}

#[derive(Serialize)]
struct ResponseStruct {
    directories: Vec<String>,
    files: Vec<ResponseFile>
}

pub async fn execute(req: HttpRequest, target_data: Path<(String, String)> ) -> HttpResponse {
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

    let root_path: String = match helper::get_root_path(&app_data.mysql, &target_data.0) {
        Ok(r) => r,
        Err(e) => return respond(e).finish(),
    };

    let target_dir = match helper::assemble_full_path(&root_path, &target_data.1) {
        Ok(r) => r,
        Err(e) => return respond(e).finish(),
    }; 

    let mut directories: Vec<String> = Vec::new();
    let mut files: Vec<ResponseFile> = Vec::new();

    for entry_res in target_dir {
        let entry = match entry_res {
            Ok(r) => r,
            Err(_) => continue
        };

        let path = entry.path();


        let file_name_os = match path.file_name() {
            Some(r) => r,
            None => continue
        };

        let file_name = match file_name_os.to_str() {
            Some(r) => String::from(r),
            None => continue
        };
        

        if path.is_dir() {
            directories.push(file_name);
        } 
        else if path.is_file() {
            let file_metadata = match entry.metadata() {
                Ok(r) => r,
                Err(_) => continue
            };

            let response_file = ResponseFile {
                name: file_name,
                size: file_metadata.len(),
            };

            files.push(response_file);
        }
    }

    let response_object = ResponseStruct {
        directories,
        files
    };


    respond(StatusCode::OK).body(
        serde_json::to_string(&response_object).unwrap()
    )
}

/*
pub async fn execute(mysql: Arc<RwLock<PooledConn>>, remote: Option<SocketAddr>, session_map: Arc<RwLock<HashMap<String,SessionData>>>, session: Option<String>, target_id: String, target_path: String) -> warp::http::Response<String> {

   

    l

    let mut directories: Vec<String> = Vec::new();
    let mut files: Vec<ResponseFile> = Vec::new();


    let target_directory = match helper_funcs::assemble_full_path(&root_path, &target_path) {
        Ok(r) => r,
        Err(e) => return generate_error_response(e)
    };

    for entry_res in target_directory {
        let entry = match entry_res {
            Ok(r) => r,
            Err(_) => continue
        };

        let path = entry.path();


        let file_name_os = match path.file_name() {
            Some(r) => r,
            None => continue
        };

        let file_name = match file_name_os.to_str() {
            Some(r) => String::from(r),
            None => continue
        };
        

        if path.is_dir() {
            directories.push(file_name);
        } 
        else if path.is_file() {
            let file_metadata = match entry.metadata() {
                Ok(r) => r,
                Err(_) => continue
            };

            let response_file = ResponseFile {
                name: file_name,
                size: file_metadata.len(),
            };

            files.push(response_file);
        }
    }


    let response_object = ResponseStruct {
        directories,
        files
    };

    info_mesg(StatusCode::OK);

    utils::create_response_builder().status(StatusCode::OK).body(
        serde_json::to_string(&response_object).unwrap()
    ).unwrap()
}*/