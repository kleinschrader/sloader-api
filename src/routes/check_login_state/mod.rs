use std::{sync::{Arc, RwLock}, collections::HashMap, net::SocketAddr, time::Instant};

use crate::utils::{create_response_builder, print_conneciton_info, session::SessionData,};

use http::StatusCode;
use mysql::PooledConn;
use serde::Serialize;

#[derive(Serialize)]
struct ResponseStruct {
    authenticated: bool
}

const METHOD: &str = "GET";
const ROUTE: &str = "/loginState";

pub async fn execute(_mysql: Arc<RwLock<PooledConn>>, remote: Option<SocketAddr>, session_map: Arc<RwLock<HashMap<String,SessionData>>>, session: Option<String>) -> warp::http::Response<String> {
    let start_time = Instant::now();
    let info_mesg = |sc: StatusCode| {
        print_conneciton_info(remote, METHOD, ROUTE, sc, start_time.elapsed())
    };

    let builder = create_response_builder();

    let session_string;

    if session.is_none() {
        let rs = ResponseStruct {
            authenticated: false
        };
        
        let response = serde_json::to_string(&rs).unwrap();

        info_mesg(StatusCode::OK);

        return builder
            .body(response)
            .unwrap();
    }
    else {
        session_string = session.unwrap();
    }

    let authenticated = session_map.read().unwrap().contains_key(&session_string);

    let rs = ResponseStruct {
        authenticated
    };

    let response = serde_json::to_string(&rs).unwrap();

    info_mesg(StatusCode::OK);

    return builder
            .body(response)
            .unwrap();
}
