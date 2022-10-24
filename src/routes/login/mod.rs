use std::{net::SocketAddr, sync::{Arc, RwLock}, collections::HashMap, time::Instant};

use bytes::Bytes;
use http::StatusCode;
use mysql::PooledConn;
use serde::{Serialize, Deserialize};
use rand::{rngs::OsRng, distributions::DistString};

use crate::utils::{self, session::SessionData, print_conneciton_info};


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

const METHOD: &str = "POST";
const ROUTE: &str = "/login";

pub async fn execute(mysql: Arc<RwLock<PooledConn>>,remote: Option<SocketAddr>, session_map: Arc<RwLock<HashMap<String,SessionData>>>, _session: Option<String>, body: Bytes) -> warp::http::Response<String> {
    let start_time = Instant::now();
    let info_mesg = |sc: StatusCode| {
        print_conneciton_info(remote, METHOD, ROUTE, sc, start_time.elapsed())
    };


    let generate_error_response = |responsecode: StatusCode| {
        info_mesg(responsecode);
        let resp = ResponseStruct{success: false, userdata: None};
        utils::create_response_builder().status(responsecode).body(
            serde_json::to_string(&resp).unwrap()
        ).unwrap()
    };

    

    let body_string = match String::from_utf8(body.to_vec()) {
        Ok(r) => r,
        Err(_) => {
            return generate_error_response(StatusCode::BAD_REQUEST);
        }
    };

    let params: RequestStruct = match serde_json::from_str(&body_string) {
        Ok(r) => r,
        Err(_) => {
            return generate_error_response(StatusCode::BAD_REQUEST);
        }
    };


    let password_params_opt;
    
    {
        // start a new closure to release write lock when we dont need it.
        let conn: &mut PooledConn = &mut mysql.write().expect("Communication failure doing runtime");
        password_params_opt = utils::mysql::get_user_password_params(conn, &params.username);
    }
      

    if password_params_opt.is_none() {
        return generate_error_response(StatusCode::UNAUTHORIZED);
    }

    let password_params = password_params_opt.unwrap();

    let verify_result = match argon2::verify_encoded(&password_params.1, &params.password.as_bytes()) {
        Ok(r) => r,
        Err(_) => {
            return generate_error_response(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };


    if verify_result == false {
        return generate_error_response(StatusCode::UNAUTHORIZED);
    }

    let mut osrng = OsRng;
    let session_key = rand::distributions::Alphanumeric.sample_string(&mut osrng, 128);

    let session_object;
    {
        // start a new closure to release write lock when we dont need it.
        let conn: &mut PooledConn = &mut mysql.write().expect("Communication failure doing runtime");
        session_object = match utils::mysql::create_session_data(conn, &params.username) {
            Some(r) => r,
            None => {
                return generate_error_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
    }
    
    let cookie_setter = format!("SESSION={}; HttpOnly",session_key);

    match session_map.write() {
        Ok(mut r) => {

            r.retain(|_, value| {
                value.userid != session_object.userid
            });
            
            r.insert(session_key.clone(), session_object.clone());
        }
        Err(_) => {
            return generate_error_response(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }


    info_mesg(StatusCode::OK);
    let resp = ResponseStruct{
        success: true,
        userdata: Some(session_object),
    };
    
    utils::create_response_builder().header("Set-Cookie", cookie_setter).body(
        serde_json::to_string(&resp).unwrap()
    ).unwrap()
}