use std::path::Path;

pub fn get_video_title(path: &Path) -> String {
    let output = std::process::Command
        ::new("ffprobe")
        .args(["-v", "quiet", "-print_format", "json", "-show_format", path.to_str().unwrap()])
        .output();

    match output {
        Ok(output) => {
            if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
                if
                    let Some(title) = json
                        .get("format")
                        .and_then(|format| format.get("tags"))
                        .and_then(|tags| tags.get("title"))
                        .and_then(|title| title.as_str())
                {
                    return title.to_string();
                }
            }
            path.file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown")
                .to_string()
        }
        Err(_) =>
            path
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown")
                .to_string(),
    }
}
