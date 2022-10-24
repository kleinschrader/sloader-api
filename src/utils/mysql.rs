use std::{env, sync::{Arc, RwLock}, convert::Infallible};

use byteorder::ByteOrder;
use log::error;
use mysql::{Pool, PooledConn, prelude::Queryable};
use warp::Filter;

use super::session::SessionData;

pub fn init_mysql() -> PooledConn {
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

    match pool.get_conn() {
        Ok(r) => r,
        Err(_) => {
            error!("Error getting pool_con. Bailing");
            panic!();
        }
    }
}

pub fn create_mysql_filter(conn: PooledConn) -> impl Filter<Extract = (Arc<RwLock<PooledConn>>,), Error = Infallible> + Clone {
    let cell = Arc::new(RwLock::new(conn));

    warp::any().map(
        move || cell.clone()
    )
}

pub fn check_if_db_is_setup(conn: &mut PooledConn) -> bool {
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

pub fn get_user_password_params(conn: &mut PooledConn, username: &str) -> Option<(String,String)> {
    let r: Vec<(String,String)> = conn.exec("SELECT Salt,Hashedpassword from Users WHERE Username = ?", (username,)).expect("IN PROGRESS DB ERROR");

    if r.len() == 1 {
        Some(r[0].to_owned())
    }
    else {
        None
    }
}

pub fn create_session_data(conn: &mut PooledConn, username: &str) -> Option<SessionData> {
    let r: Vec<(Vec<u8>,String,bool)> = conn.exec("SELECT UserID,FullName,Administrator from Users WHERE Username = ? LIMIT 1", (username,)).expect("IN PROGRESS DB ERROR");

    if r.len() == 1 {
        let userid = byteorder::BigEndian::read_u128(r[0].0.as_ref());
        
        Some(
            SessionData{
                userid,
                name: r[0].1.to_owned(),
                admin: r[0].2,
            }
        )
    }
    else {
        None
    }
}