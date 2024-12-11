use std::process::Command;

#[derive(PartialEq)]
#[allow(dead_code)]
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
            VideoEncoder::Nvenc => vec!["-c:v", "h264_nvenc"],
            VideoEncoder::Amf => vec!["-c:v", "h264_amf"],
            VideoEncoder::QuickSync => vec!["-c:v", "h264_qsv"],
            VideoEncoder::VideoToolbox => vec!["-c:v", "h264_videotoolbox"],
            VideoEncoder::Software => vec!["-c:v", "libx264"],
        }
    }
}

pub fn get_hardware_config() -> HardwareConfig {
    HardwareConfig::detect()
}
