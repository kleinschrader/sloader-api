use std::{env, sync::{Arc, RwLock}};

use log::error;
use mysql::{Pool, PooledConn, prelude::Queryable};

pub fn init_mysql() -> Arc<std::sync::RwLock<PooledConn>> {
    // mysql init here
    let url = match env::var("MYSQL_URL") {
        Ok(r) => r,
        Err(_) => {
            error!("Error loading MYSQL_URL env var. Bailing");
            panic!();
        }
    };

    let opts = match mysql::Opts::from_url(&url) {
        Ok(r) => r,
        Err(_) => {
            error!("Error parsing MYSQL_URL env var. Bailing");
            panic!();
        }
    };

    let pool = match Pool::new(opts) {
        Ok(r) => r,
        Err(_) => {
            error!("Error initializing pool. Bailing");
            panic!();
        }
    };

    let conn = match pool.get_conn() {
        Ok(r) => r,
        Err(_) => {
            error!("Error getting pool_con. Bailing");
            panic!();
        }
    };

    Arc::new(RwLock::new(conn))
}

pub fn check_if_db_is_setup(mysql: &mut Arc<std::sync::RwLock<PooledConn>>) -> bool {
    
    let mut conn = mysql.write().expect("Unable to get mysql write log");

    let r: Vec<String> = match conn.query("SHOW DATABASES LIKE 'sloader';") {
        Ok(r) => r,
        Err(e) => {
            error!("Unable to execute db lookup: {}", e);
            panic!();
        }
    };

    if r.len() > 0 {
        true
    }
    else {
        false
    }
}
