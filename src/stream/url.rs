use warp::Filter;
use std::{ fs, net::SocketAddr };

use crate::stream::events::events;

pub struct StreamServer {
    port: u16,
    addr: SocketAddr,
}

impl StreamServer {
    pub fn new(port: u16) -> Self {
        StreamServer {
            port,
            addr: ([0, 0, 0, 0], port).into(),
        }
    }

    pub fn url(&self) -> String {
        format!("http://0.0.0.0:{}/stream/output.m3u8", self.port)
    }

    pub async fn start(&self) {
        if fs::metadata("stream").is_ok() {
            fs::remove_dir_all("stream").expect("Failed to delete 'stream' directory");
        }

        // Recreate the "stream" folder
        fs::create_dir_all("stream").expect("Failed to create 'stream' directory");

        let cors: warp::filters::cors::Builder = warp
            ::cors()
            .allow_any_origin()
            .allow_header("content-type")
            .allow_methods(vec!["GET", "POST"]);

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

        let routes = specific_route.or(other_routes).with(cors);

        println!("Stream URL: {}", self.url());

        warp::serve(routes).run(self.addr).await;
    }
}
