use std::sync::{Arc, RwLock};

use byteorder::ByteOrder;
use mysql::{PooledConn, prelude::Queryable};
use serde::Serialize;

#[derive(Serialize)]
pub struct Target {
    pub target_id: String,
    pub nick_name: String,
    pub target_path: String,
}

pub fn fetch_targets(mysql: Arc<RwLock<PooledConn>>) -> Option<Vec<Target>> {
    let mut conn = match mysql.write() {
        Ok(r) => r,
        Err(_) => return None,
    };

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


    Some(parsed_targets)
}