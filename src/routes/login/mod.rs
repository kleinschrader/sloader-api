

use actix_web::{HttpRequest, HttpResponse};
use bytes::Bytes;
use http::StatusCode;
use serde::{Serialize, Deserialize};
use rand::{rngs::OsRng, distributions::DistString};

use crate::{utils::{session::SessionData, method_logger::MethodLogger, create_response_builder}, app_structs::AppData};


#[derive(Serialize)]
struct ResponseStruct {
    success: bool,
    userdata: Option<SessionData>
}

#[derive(Deserialize)]
struct RequestStruct {
    username: String,
    password: String,
}

mod helper;

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

    let const_failure_resp = ResponseStruct {
        success: false,
        userdata: None
    };
    


    let request_data: RequestStruct = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(_) => return respond(StatusCode::BAD_REQUEST).finish(),
    };

    let password_params = match helper::get_user_password_params(app_data.mysql.clone(), &request_data.username) {
        Some(r) => r,
        None => return respond(StatusCode::UNAUTHORIZED).body({
            serde_json::to_string(&const_failure_resp).unwrap()
        }),
    };

    let password_matches = match argon2::verify_encoded(&password_params.1, &request_data.password.as_bytes()) {
        Ok(r) => r,
        Err(_) => return respond(StatusCode::UNAUTHORIZED).body({
            serde_json::to_string(&const_failure_resp).unwrap()
        }),
    };

    if password_matches == false {
        return respond(StatusCode::UNAUTHORIZED).body({
            serde_json::to_string(&const_failure_resp).unwrap()
        });
    }

    let mut osrng = OsRng;
    let session_key = rand::distributions::Alphanumeric.sample_string(&mut osrng, 128);

    let session_object = match helper::create_session_data(app_data.mysql.clone(), &request_data.username) {
        Some(r) => r,
        None => return respond(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    };

    if app_data.insert_session(&session_key, session_object.clone()).is_err() {
        return respond(StatusCode::INTERNAL_SERVER_ERROR).finish()
    };

    let resp = ResponseStruct {
        success: true,
        userdata: Some(session_object)
    };
    
    let session_cookie = helper::create_session_cookie(&session_key);

    respond(StatusCode::OK)
        .cookie(session_cookie)
        .body(
        serde_json::to_string(&resp).unwrap()
    )
}