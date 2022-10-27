use std::{sync::{Arc, RwLock}, collections::HashMap};

use actix_web::HttpRequest;
use mysql::PooledConn;

use crate::utils::session::SessionData;

#[derive(Clone)]
pub struct AppData {
    pub mysql: Arc<RwLock<PooledConn>>,
    pub session_map:Arc<RwLock<HashMap<String,Arc<SessionData>>>>
}

impl AppData {
    pub fn get_session(&self, req: &HttpRequest) -> Option<Arc<SessionData>> {
        let session_id_cookie = req.cookie("SESSION")?;
        let session_id = session_id_cookie.value();
        
        let sm = match self.session_map.read() {
            Ok(r) => r,
            Err(_) => return None
        };

        match sm.get(session_id) {
            Some(r) => Some(r.clone()),
            None => None,
        }
    }

    pub fn insert_session(&self, session_id: &str, session: SessionData) -> Result<(),()> {

        let mut sm = match self.session_map.write() {
            Ok(r) => r,
            Err(_) => return Err(()),
        };

        sm.insert(session_id.to_string(), Arc::new(session));

        Ok(())
    }
}