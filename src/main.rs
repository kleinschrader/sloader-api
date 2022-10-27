use std::{env, io};

use log::{warn, error, info};

mod routes;
mod utils;

mod app_structs;

#[tokio::main]
async fn main() -> io::Result<()> {
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

    let mut mysql = utils::mysql::init_mysql();
    let session_map = utils::session::create_session_map();

    match utils::mysql::check_if_db_is_setup(&mut mysql) {
        true => info!("DB seems to be already setup"),
        false => {
            error!("DB is NOT setup! place init.sql in db");
            panic!();
        }
    }

    let app_data = app_structs::AppData {
        mysql,
        session_map
    };


    actix_web::HttpServer::new(
        move || actix_web::App::new()
            .app_data(app_data.clone())
            .service(
                actix_web::web::resource("/loginState")
                    .route(actix_web::web::get().to(routes::check_login_state::execute))
            )
            .service(
                actix_web::web::resource("/login")
                    .route(actix_web::web::post().to(routes::login::execute))
            )
            .service(
                actix_web::web::resource("/targets")
                    .route(actix_web::web::get().to(routes::targets::execute))
            )
            .service(
                actix_web::web::resource("/contents/{target_id}/{path}")
                    .route(actix_web::web::get().to(routes::contents::execute))
            )
            .service(
                actix_web::web::resource("/download/{target_id}/{path}")
                    .route(actix_web::web::get().to(routes::download::execute))
            )
            .service(
                actix_web::web::resource("/createTarget")
                    .route(actix_web::web::post().to(routes::create_target::execute))
            )
            .service(
                actix_web::web::resource("/directories/{path}")
                    .route(actix_web::web::get().to(routes::directory_list::execute))
            )
            .service(
                actix_web::web::resource("/logout")
                    .route(actix_web::web::get().to(routes::logout::execute))
            )
            .service(
                actix_web::web::resource("/upload/{target_id}/{path}")
                    .route(actix_web::web::post().to(routes::upload::execute))
            )
            .default_service(
                actix_web::web::to(routes::not_found::execute)
            )
    ).bind(("127.0.0.1",3030))?.run().await?;

    Ok(())
}
