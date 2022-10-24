use std::{net::SocketAddr, sync::{Arc, RwLock}, collections::HashMap, time::Instant};

use http::StatusCode;
use mysql::PooledConn;

use crate::utils::{session::SessionData, self, print_conneciton_info};

const METHOD: &str = "GET";
const ROUTE: &str = "/logout";

pub async fn execute(_mysql: Arc<RwLock<PooledConn>>,remote: Option<SocketAddr>, session_map: Arc<RwLock<HashMap<String,SessionData>>>, session: Option<String>) -> warp::http::Response<String> {
    let start_time = Instant::now();
    let info_mesg = |sc: StatusCode| {
        print_conneciton_info(remote, METHOD, ROUTE, sc, start_time.elapsed())
    };

    let session_key = match session {
        Some(r) => r,
        None => {
            info_mesg(StatusCode::OK);
            return utils::create_response_builder().body(
                String::from("")
            ).unwrap();
        }
    };

    match session_map.write() {
        Ok(mut r) => {
            r.retain(|key, _|{
                key.ne(&session_key)
            })
        },
        Err(_) => {
            info_mesg(StatusCode::INTERNAL_SERVER_ERROR);
            return utils::create_response_builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(
                String::from("")
            ).unwrap();
        }
    }


    info_mesg(StatusCode::OK);
    utils::create_response_builder().body(
        String::from("")
    ).unwrap()
}