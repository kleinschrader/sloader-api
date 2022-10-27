use std::{collections::HashMap, sync::{RwLock, Arc}};
use serde::Serialize;

//TODO Currently we have to clone the sessiondata if we went to use it or use them nestely. We should wrap the im cell or somethind idk

#[derive(Serialize,Clone)]
pub struct SessionData {
    pub userid: u128,
    pub name: String,
    pub admin: bool,
}

pub fn create_session_map() -> Arc<RwLock<HashMap<String,Arc<SessionData>>>> {
    let session_storage = HashMap::<String,Arc<SessionData>>::new();
    let session_storage_cell = RwLock::new(session_storage);

    Arc::new(session_storage_cell)
}