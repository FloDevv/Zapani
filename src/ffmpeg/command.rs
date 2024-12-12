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
        "+nobuffer+fastseek+genpts+igndts",
        "-thread_queue_size",
        "4096",
        "-f",
        "concat",
        "-safe",
        "0",
        "-i",
        list_path.to_str().unwrap(),
        "-map",
        "0:v", // Map first video stream
        "-map",
        "0:a:0", // Map first audio stream
        "-profile:v",
        "main",
        "-preset",
        "medium",
        "-g",
        "60",
        "-b:v",
        "5000k",
        "-maxrate",
        "6000k",
        "-bufsize",
        "10000k",
        "-c:a",
        "aac",
        "-b:a",
        "192k",
        "-af",
        "aresample=async=1000",
        "-ar",
        "48000",
        "-ac",
        "2",
        "-metadata:s:a:0",
        "language=fra", // Still set French language tag
        "-max_muxing_queue_size",
        "1024",
        "-f",
        "hls",
        "-hls_time",
        "4",
        "-hls_list_size",
        "10",
        "-hls_flags",
        "delete_segments+omit_endlist+discont_start",
        "-hls_segment_type",
        "mpegts",
        "-hls_segment_filename",
        "stream/segment%d.ts",
        "stream/output.m3u8"
    ];

    args.splice(14..14, encoder_params);

    if !use_hardware {
        args.push("-tune");
        args.push("zerolatency");
    }

    cmd(args[0], &args[1..])
}
