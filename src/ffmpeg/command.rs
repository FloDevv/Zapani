use duct::cmd;
use std::path::Path;
use super::hardware;

pub fn create_ffmpeg_command(list_path: &Path) -> duct::Expression {
    let hw_config: hardware::HardwareConfig = hardware::get_hardware_config();
    let encoder_params: Vec<&str> = hw_config.get_ffmpeg_params();
    let use_hardware: bool = hw_config.encoder != hardware::VideoEncoder::Software;

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
        list_path.to_str().unwrap(),
        "-profile:v",
        "main",
        "-preset",
        "medium",
        "-g",
        "48",
        "-sc_threshold",
        "0",
        "-b:v",
        "2500k",
        "-maxrate",
        "2800k",
        "-bufsize",
        "5600k",
        "-c:a",
        "copy",
        "-ac",
        "2",
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
        "stream/output.m3u8"
    ];

    // Insert encoder parameters
    args.splice(20..20, encoder_params);

    // Conditionally add '-tune zerolatency' if not using hardware encoder
    if !use_hardware {
        args.push("-tune");
        args.push("zerolatency");
    }

    cmd(args[0], &args[1..])
}
