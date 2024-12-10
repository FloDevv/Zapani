// generator.rs
use std::fs::File;
use std::io::Write;
use std::path::{ Path, PathBuf };
use walkdir::WalkDir;

use super::customrandom::customrandom;

pub fn create_playlist(sources: &[String], output_path: &str) -> std::io::Result<()> {
    let mut video_files: Vec<PathBuf> = scan_video_files(sources);
    customrandom(&mut video_files);
    write_playlist_file(&video_files, output_path)
}

fn scan_video_files(sources: &[String]) -> Vec<PathBuf> {
    sources
        .iter()
        .flat_map(|dir: &String| {
            WalkDir::new(dir)
                .into_iter()
                .filter_map(|e: Result<walkdir::DirEntry, walkdir::Error>| e.ok())
                .map(|e: walkdir::DirEntry| e.path().to_path_buf())
                .filter(|path: &PathBuf| is_video_file(path))
        })
        .collect()
}

fn write_playlist_file(videos: &[PathBuf], output_path: &str) -> std::io::Result<()> {
    let mut file: File = File::create(output_path)?;
    for video in videos {
        let path_str: String = video.to_str().unwrap().replace("\\", "\\\\");
        writeln!(file, "file '{}'", path_str)?;
    }
    Ok(())
}

fn is_video_file(path: &Path) -> bool {
    match path.extension().and_then(|ext: &std::ffi::OsStr| ext.to_str()) {
        Some(ext) => ["mp4", "mkv", "avi"].contains(&ext.to_lowercase().as_str()),
        None => false,
    }
}
