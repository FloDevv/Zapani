use serde::Deserialize;
use std::fs;
use stream::StreamServer;
mod ffmpeg;
mod stream;
mod playlist;

#[derive(Deserialize)]
struct Config {
    sources: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_str: String = fs::read_to_string("sources.json")?;
    let config: Config = serde_json::from_str(&config_str)?;

    let list_path: &str = "video.json";
    let entries: Vec<playlist::VideoEntry> = playlist::create_playlist(&config.sources, list_path)?;
    playlist::write_playlist_file(&entries, list_path)?;

    let concat_path: &str = "video.txt";
    playlist::write_ffmpeg_concat_file(&entries, concat_path)?;

    let server_handle: tokio::task::JoinHandle<()> = tokio::spawn(async {
        let server: StreamServer = StreamServer::new(8080);
        server.start().await
    });

    let ffmpeg_command = ffmpeg
        ::create_ffmpeg_command(std::path::Path::new(concat_path))
        .stdout_null()
        .stderr_to_stdout()
        .start()?;

    ffmpeg_command.wait()?;

    match server_handle.await {
        Ok(()) => (),
        Err(e) => eprintln!("Server error: {}", e),
    }

    Ok(())
}
