use duct::cmd;
use std::path::Path;
use super::hardware;

pub fn create_ffmpeg_command(list_path: &Path) -> duct::Expression {
    let hw_config: hardware::HardwareConfig = hardware::get_hardware_config();
    let encoder_params: Vec<&str> = hw_config.get_ffmpeg_params();

    let mut args: Vec<&str> = vec![
        "ffmpeg",
        "-stream_loop",
        "-1",
        "-re",
        "-fflags",
        "+nobuffer+fastseek",
        "-flags",
        "low_delay",
        "-probesize",
        "32",
        "-analyzeduration",
        "0",
        "-thread_queue_size",
        "512",
        "-f",
        "concat",
        "-safe",
        "0",
        "-i",
        list_path.to_str().unwrap()
    ];

    // Add hardware-specific encoding parameters
    args.extend(encoder_params.iter().map(|&s| s));

    // Add remaining parameters
    args.extend_from_slice(
        &[
            "-profile:v",
            "baseline",
            "-c:a",
            "aac",
            "-b:a",
            "128k",
            "-ac",
            "2",
            "-map",
            "0:v:0",
            "-map",
            "0:a:0",
            "-f",
            "hls",
            "-hls_time",
            "4",
            "-hls_list_size",
            "5",
            "-hls_flags",
            "delete_segments+omit_endlist+discont_start",
            "-hls_segment_type",
            "mpegts",
            "-hls_segment_filename",
            "stream/segment%d.ts",
            "-hls_allow_cache",
            "1",
            "-g",
            "120",
            "-keyint_min",
            "120",
            "-sc_threshold",
            "0",
            "-bufsize",
            "8192k",
            "stream/output.m3u8",
        ]
    );

    cmd(args[0], &args[1..])
}
