use std::env;
use std::net::SocketAddr;
use std::time::Duration;

use http::response::Builder;
use http::StatusCode;
use log::info;

pub mod session;
pub mod mysql;

pub fn create_response_builder() -> Builder {
    let cors_allow: String = env::var("CORS_ALLOW_ORIGIN").unwrap();

    Builder::new()
        .header("Access-Control-Allow-Origin", cors_allow)
        .header("Access-Control-Allow-Credentials", "true")
        .header("Content-Type", "application/json")
}

pub fn print_conneciton_info(remote: Option<SocketAddr>, method: &'static str, routename: &'static str, status: StatusCode, elapsed_time: Duration) {
    let remote_ip = match remote {
        Some(r) => r.to_string(),
        None => String::from("?.?.?.?:????")
    };

    info!("[{}][{}ms] {} {} {}", status.as_u16(), elapsed_time.as_millis(), remote_ip, method, routename);
}

#[cfg(test)]
mod tests {
    use http::StatusCode;

    #[test]
    fn create_response_builder_expected() {
        let result = super::create_response_builder();
        
        let response = result.body(String::from("test")).unwrap();
    
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body(), "test");
        assert_eq!(response.headers()["Content-Type"], "application/json");
    }
}