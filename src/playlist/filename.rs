use std::fs;
use std::path::PathBuf;

pub fn sanitize_filenames(files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for i in 0..files.len() {
        let original_path: &PathBuf = &files[i];
        if
            let Some(file_name) = original_path
                .file_name()
                .and_then(|n: &std::ffi::OsStr| n.to_str())
        {
            if file_name.contains('\'') {
                let sanitized_name: String = file_name
                    .chars()
                    .filter(|&c| c != '\'')
                    .collect();
                let sanitized_path: PathBuf = original_path.with_file_name(sanitized_name);

                fs::rename(original_path, &sanitized_path)?;
                files[i] = sanitized_path;
            }
        }
    }
    Ok(())
}
