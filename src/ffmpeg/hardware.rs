use std::process::Command;

#[derive(PartialEq)]
pub enum VideoEncoder {
    Nvenc,
    Amf,
    QuickSync,
    VideoToolbox,
    Software,
}

pub struct HardwareConfig {
    pub encoder: VideoEncoder,
    pub gpu_brand: Option<String>,
}

impl HardwareConfig {
    pub fn detect() -> Self {
        let mut config: HardwareConfig = HardwareConfig {
            encoder: VideoEncoder::Software,
            gpu_brand: None,
        };

        // Check if MacOS
        #[cfg(target_os = "macos")]
        {
            config.encoder = VideoEncoder::VideoToolbox;
            return config;
        }

        // For Windows/Linux, check GPU
        if cfg!(target_os = "windows") {
            // Check for NVIDIA GPU
            if
                let Ok(output) = Command::new("wmic")
                    .args(["path", "win32_videocontroller", "get", "name"])
                    .output()
            {
                let output: String = String::from_utf8_lossy(&output.stdout).to_lowercase();
                if output.contains("nvidia") {
                    config.encoder = VideoEncoder::Nvenc;
                    config.gpu_brand = Some("NVIDIA".to_string());
                } else if output.contains("amd") {
                    config.encoder = VideoEncoder::Amf;
                    config.gpu_brand = Some("AMD".to_string());
                } else if output.contains("intel") {
                    config.encoder = VideoEncoder::QuickSync;
                    config.gpu_brand = Some("Intel".to_string());
                }
            }
        }

        config
    }

    pub fn get_ffmpeg_params(&self) -> Vec<&'static str> {
        match self.encoder {
            VideoEncoder::Nvenc =>
                vec!["-c:v", "h264_nvenc", "-preset", "p5", "-rc", "vbr", "-cq", "23"],
            VideoEncoder::Amf => vec!["-c:v", "h264_amf", "-quality", "balanced", "-rc", "vbr_lat"],
            VideoEncoder::QuickSync =>
                vec!["-c:v", "h264_qsv", "-preset", "medium", "-global_quality", "23"],
            VideoEncoder::VideoToolbox =>
                vec![
                    "-c:v",
                    "h264_videotoolbox",
                    "-q:v",
                    "23",
                    "-realtime",
                    "1",
                    "-threads",
                    "auto"
                ],
            VideoEncoder::Software =>
                vec!["-c:v", "libx264", "-preset", "medium", "-tune", "zerolatency"],
        }
    }
}

pub fn get_hardware_config() -> HardwareConfig {
    HardwareConfig::detect()
}
