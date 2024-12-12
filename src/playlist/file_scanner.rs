use std::path::{ Path, PathBuf };
use walkdir::WalkDir;

pub fn scan_video_files(sources: &[String]) -> Vec<PathBuf> {
    sources
        .iter()
        .flat_map(|dir| {
            WalkDir::new(dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .map(|e| e.path().to_path_buf())
                .filter(|path| is_video_file(path))
        })
        .collect()
}

pub fn is_video_file(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => ["mp4", "mkv", "avi"].contains(&ext.to_lowercase().as_str()),
        None => false,
    }
}
