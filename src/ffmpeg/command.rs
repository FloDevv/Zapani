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
        "+nobuffer+fastseek+genpts",
        "-thread_queue_size",
        "4096",
        "-f",
        "concat",
        "-safe",
        "0",
        "-i",
        list_path.to_str().unwrap(),
        "-profile:v",
        "main",
        "-preset",
        "fast",
        "-g",
        "40",
        "-b:v",
        "3000k",
        "-maxrate",
        "3500k",
        "-bufsize",
        "6000k",
        "-c:a",
        "copy",
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
