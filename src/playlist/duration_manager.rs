use chrono::Duration;
use std::path::Path;
use std::process::Command;

const SEVEN_DAYS_SECONDS: i64 = 7 * 24 * 60 * 60;

pub struct PlaylistDurationManager {
    target_duration: Duration,
    current_duration: Duration,
}

impl PlaylistDurationManager {
    pub fn new() -> Self {
        Self {
            target_duration: Duration::seconds(SEVEN_DAYS_SECONDS),
            current_duration: Duration::seconds(0),
        }
    }

    pub fn needs_more_content(&self) -> bool {
        self.current_duration < self.target_duration
    }

    pub fn add_duration(&mut self, duration: f64) {
        self.current_duration = self.current_duration + Duration::seconds(duration as i64);
    }
}

pub fn get_video_duration(path: &Path) -> Option<f64> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            path.to_str().unwrap(),
        ])
        .output()
        .ok()?;

    String::from_utf8(output.stdout).ok()?.trim().parse().ok()
}
