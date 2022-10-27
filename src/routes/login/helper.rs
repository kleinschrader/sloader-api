use std::sync::{Arc, RwLock};

use actix_web::cookie::{Cookie, CookieBuilder};
use byteorder::ByteOrder;
use mysql::{PooledConn, prelude::Queryable};

use crate::utils::session::SessionData;


pub fn get_user_password_params(mysql: Arc<RwLock<PooledConn>>, username: &str) -> Option<(String,String)> {
    let mut conn = match mysql.write() {
        Ok(r) => r,
        Err(_) => return None
    };

    let r: Vec<(String,String)> = conn.exec("SELECT Salt,Hashedpassword from Users WHERE Username = ?", (username,)).expect("IN PROGRESS DB ERROR");

    if r.len() == 1 {
        Some(r[0].to_owned())
    }
    else {
        None
    }
}

pub fn create_session_data(mysql: Arc<RwLock<PooledConn>>, username: &str) -> Option<SessionData> {
    let mut conn = match mysql.write() {
        Ok(r) => r,
        Err(_) => return None
    };

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

pub fn create_session_cookie(session_key: &str) -> Cookie {
    CookieBuilder::new("SESSION", session_key)
        .http_only(true)
        .finish()
}