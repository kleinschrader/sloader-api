use std::{sync::{RwLock, Arc}, fs::ReadDir, str::FromStr};

use http::StatusCode;
use log::{error, warn};
use mysql::{PooledConn, prelude::Queryable};

use path_clean::PathClean;
use uuid::Uuid;

pub fn get_root_path(mysql: &Arc<RwLock<PooledConn>>, target_id: &str) -> Result<String, StatusCode>{
    
    let uuid = match Uuid::from_str(target_id) {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };


    let conn: &mut PooledConn = &mut mysql.write().expect("Communication failure doing runtime");
            
    let db_result: Vec<(String,)> = match conn.exec("SELECT TargetPath FROM Targets WHERE TargetID = (?)", (uuid.as_bytes(),)) {
        Ok(r) => r,
        Err(e) => {
            error!("DB ERROR: {}",e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    if db_result.len() != 1 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(String::from(&db_result[0].0))
}

pub fn assemble_full_path(root_path: &str, sub_path: &str) -> Result<ReadDir, StatusCode> {

    let relative_path_raw = std::path::PathBuf::from(&sub_path);

    let relative_path_clean = relative_path_raw.clean();

    let relative_path = relative_path_clean.strip_prefix("/").unwrap_or(
        relative_path_clean.as_path()
    );

    let relative_path_str = match relative_path.to_str() {
        Some(r) => r,
        None => return {
            warn!("Unabled to convert path to string");
            Err(StatusCode::BAD_REQUEST)
        }
    };

    let mut res_string = String::from(root_path);

    res_string.push_str(relative_path_str);

    let dir = match std::fs::read_dir(res_string) {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::NOT_FOUND)
    };

    Ok(dir)
}