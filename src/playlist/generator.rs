use std::fs::File;
use std::path::PathBuf;
use chrono::{ DateTime, Utc, Duration };

use crate::playlist::{
    video_entry::VideoEntry,
    file_scanner::scan_video_files,
    metadata::get_video_title,
};
use super::{
    customrandom::customrandom,
    filename::sanitize_filenames,
    duration_manager::{ PlaylistDurationManager, get_video_duration },
};

pub fn create_playlist(sources: &[String], output_path: &str) -> std::io::Result<()> {
    let mut video_files: Vec<PathBuf> = scan_video_files(sources);
    sanitize_filenames(&mut video_files)?;
    let original_files: Vec<PathBuf> = video_files.clone();

    let mut entries: Vec<VideoEntry> = Vec::new();
    let mut current_time: DateTime<Utc> = Utc::now();
    let mut duration_manager: PlaylistDurationManager = PlaylistDurationManager::new();

    while duration_manager.needs_more_content() {
        if video_files.is_empty() {
            video_files = original_files.clone();
            customrandom(&mut video_files);
        }

        if let Some(video) = video_files.pop() {
            if let Some(duration) = get_video_duration(&video) {
                let entry: VideoEntry = VideoEntry {
                    path: video.to_str().unwrap().replace("\\", "\\\\"),
                    title: get_video_title(&video),
                    start_time: current_time,
                    duration,
                };
                current_time = current_time + Duration::seconds(duration as i64);
                duration_manager.add_duration(duration);
                entries.push(entry);
            }
        }
    }

    write_playlist_file(&entries, output_path)
}

fn write_playlist_file(entries: &[VideoEntry], output_path: &str) -> std::io::Result<()> {
    if PathBuf::from(output_path).exists() {
        std::fs::remove_file(output_path)?;
    }
    let file: File = File::create(output_path)?;
    serde_json::to_writer_pretty(file, &entries)?;
    Ok(())
}
