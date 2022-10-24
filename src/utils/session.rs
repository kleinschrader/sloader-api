use std::{collections::HashMap, sync::{RwLock, Arc}, convert::Infallible};
use serde::Serialize;
use warp::Filter;

//TODO Currently we have to clone the sessiondata if we went to use it or use them nestely. We should wrap the im cell or somethind idk

#[derive(Serialize,Clone)]
pub struct SessionData {
    pub userid: u128,
    pub name: String,
    pub admin: bool,
}

fn create_session_map() -> Arc<RwLock<HashMap<String,SessionData>>> {
    let session_storage = HashMap::<String,SessionData>::new();
    let session_storage_cell = RwLock::new(session_storage);

    Arc::new(session_storage_cell)
}

pub fn create_session_filter() -> impl Filter<Extract = (Arc<RwLock<HashMap<String,SessionData>>>,), Error = Infallible> + Clone {
    let session_map = create_session_map();

    warp::any().map(
        move || session_map.clone()
    )
}