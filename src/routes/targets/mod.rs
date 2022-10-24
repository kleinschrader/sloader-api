use std::{sync::{Arc, RwLock}, net::SocketAddr, collections::HashMap, time::Instant};

use byteorder::ByteOrder;
use http::StatusCode;
use mysql::{PooledConn, prelude::Queryable};
use serde::Serialize;

use crate::utils::{session::SessionData, self, print_conneciton_info};

#[derive(Serialize)]
struct Target {
    pub target_id: String,
    pub nick_name: String,
    pub target_path: String,
}

#[derive(Serialize)]
struct ResponseStruct {
    pub targets: Vec<Target>
}

const METHOD: &str = "GET";
const ROUTE: &str = "/targets";

pub async fn execute(mysql: Arc<RwLock<PooledConn>>,remote: Option<SocketAddr>, session_map: Arc<RwLock<HashMap<String,SessionData>>>, session: Option<String>) -> warp::http::Response<String> {
    let start_time = Instant::now();

    let info_mesg = |sc: StatusCode| {
        print_conneciton_info(remote, METHOD, ROUTE, sc, start_time.elapsed())
    };

    let generate_error_response = |responsecode: StatusCode| {
        info_mesg(responsecode);
        utils::create_response_builder().status(responsecode).body(
            serde_json::to_string("").unwrap()
        ).unwrap()
    };


    let session_key = match session {
        Some(r) => r,
        None => {
            return generate_error_response(StatusCode::UNAUTHORIZED);
        }
    };

    let session;
    {
        match session_map.read() {
            Ok(r) => {
                match r.get(&session_key) {
                    Some(r) => {session = r.clone()},
                    None => {
                        return generate_error_response(StatusCode::UNAUTHORIZED);
                    }
                };
            }
            Err(_) => {
                return generate_error_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    if session.admin == true {
        {
            // start a new closure to release write lock when we dont need it.
            let conn: &mut PooledConn = &mut mysql.write().expect("Communication failure doing runtime");
            
            let targets: Vec<(Vec<u8>, String, String)> = conn.query("SELECT * FROM Targets").expect("IN PROGRESS DB ERROR");

            let mut parsed_targets: Vec<Target> = Vec::new();

            for i_targets in targets {
                let target_id_bin = byteorder::BigEndian::read_u128(i_targets.0.as_ref());

                let target_id = uuid::Builder::from_u128(target_id_bin).into_uuid().to_string();
                let nick_name = i_targets.1;
                let target_path = i_targets.2;

                let tgt = Target{
                    target_id,
                    nick_name,
                    target_path
                };

                parsed_targets.push(tgt);
            }

            let resp = ResponseStruct {
                targets: parsed_targets,
            };

            info_mesg(StatusCode::OK);
            return utils::create_response_builder().body(
                serde_json::to_string(&resp).unwrap()
            ).unwrap();
        }
    }
    
    //TODO Impement non Admin target getting

    info_mesg(StatusCode::OK);
    utils::create_response_builder().body(
        String::from("")
    ).unwrap()
}