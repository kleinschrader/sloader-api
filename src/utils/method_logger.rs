use std::{net::SocketAddr, time::Instant};

use actix_web::HttpRequest;
use http::StatusCode;
use log::info;

pub struct MethodLogger<'a> {
    method: &'a str,
    route: &'a str,
    peer: Option<SocketAddr>,
    instance_time: Instant,
}

impl<'a> MethodLogger<'a>{
    pub fn begin(req: &'a HttpRequest) -> Self {
        MethodLogger{
            method: req.method().as_str(),
            route: req.path(),
            peer: req.peer_addr(),
            instance_time: Instant::now()
        }
    }

    pub fn finish(&self, sc: StatusCode) {
        let remote_ip = match self.peer {
            Some(r) => r.to_string(),
            None => String::from("?.?.?.?:????")
        };

        info!("[{}][{}ms] {} {} {}", sc, self.instance_time.elapsed().as_millis(), remote_ip, self.method, self.route);
    }
}