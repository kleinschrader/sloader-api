use std::env;

use warp::Filter;
use log::{warn, error, info};

mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let _ = simplelog::TermLogger::init(simplelog::LevelFilter::Info, simplelog::Config::default(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto);

    match dotenv::dotenv() {
        Ok(_) => {},
        Err(_) => warn!("Error loading .env continuing")
    }

    match env::var("CORS_ALLOW_ORIGIN") {
        Ok(_) => {},
        Err(_) => {
            error!("Error loading CORS_ALLOW_ORIGIN env var. Bailing");
            panic!();
        }
    }

    let mut mysql_conn = utils::mysql::init_mysql();

    match utils::mysql::check_if_db_is_setup(&mut mysql_conn) {
        true => info!("DB seems to be already setup"),
        false => {
            error!("DB is NOT setup! place init.sql in db");
            panic!();
        }
    }
    


    let mysql_filter = utils::mysql::create_mysql_filter(mysql_conn);    
    let remote_filter = warp::filters::addr::remote();
    let session_map_filter = utils::session::create_session_filter();
    let session_map = warp::filters::cookie::optional("SESSION");

    let info_filters = mysql_filter.and(remote_filter).and(session_map_filter).and(session_map);

    let check_login_state_route = warp::filters::method::get()
        .and(warp::path("loginState"))
        .and(warp::path::end())
        .and(info_filters.clone())
        .then(routes::check_login_state::execute);


    let login_route = warp::filters::method::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(info_filters.clone())
        .and(warp::body::bytes())
        .then(routes::login::execute);

    let logout_route = warp::filters::method::get()
        .and(warp::path("logout"))
        .and(warp::path::end())
        .and(info_filters.clone())
        .then(routes::logout::execute);

    let get_targets_route = warp::filters::method::get()
        .and(warp::path("targets"))
        .and(warp::path::end())
        .and(info_filters.clone())
        .then(routes::targets::execute);

    let routes = check_login_state_route
        .or(login_route)
        .or(logout_route)
        .or(get_targets_route);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
