use warp::Filter;
use std::net::SocketAddr;

use crate::stream::events::events;

pub struct StreamServer {
    port: u16,
    addr: SocketAddr,
}

impl StreamServer {
    pub fn new(port: u16) -> Self {
        StreamServer {
            port,
            addr: ([127, 0, 0, 1], port).into(),
        }
    }

    pub fn url(&self) -> String {
        format!("http://localhost:{}/stream/output.m3u8", self.port)
    }

    pub async fn start(&self) {
        std::fs::create_dir_all("stream").unwrap();

        let specific_route = warp
            ::path("stream")
            .and(warp::path("output.m3u8"))
            .and(warp::path::end())
            .and(warp::fs::file("stream/output.m3u8"))
            .map(|file: warp::filters::fs::File| {
                events();
                warp::reply::with_header(file, "Content-Type", "application/vnd.apple.mpegurl")
            });

        let other_routes = warp::path("stream").and(warp::fs::dir("stream"));

        let routes = specific_route.or(other_routes);

        println!("Stream URL: {}", self.url());

        warp::serve(routes).run(self.addr).await;
    }
}
