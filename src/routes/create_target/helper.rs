use std::sync::{RwLock, Arc};

use mysql::{PooledConn, prelude::Queryable};
use uuid::Uuid;

pub fn insert_target(mysql: Arc<RwLock<PooledConn>>,name: &str, path: &str, uuid: Uuid) -> Result<(),()>{
    let conn: &mut PooledConn = &mut mysql.write().expect("Communication failure doing runtime");
    
    match conn.exec_drop("INSERT INTO Targets VALUES (?,?,?)", (uuid.as_bytes(), name, path)) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}