mod engine;
pub mod lexer;
pub mod media;
pub mod parser;

use media::frame::{Frame, PixelFormat};
use std::io::Read;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn load_image_rgb(
    path: &str,
) -> Result<(Vec<u8>, u32, u32, PixelFormat), Box<dyn std::error::Error>> {
    let probe_output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width,height",
            "-of",
            "csv=p=0",
            path,
        ])
        .output()?;

    if !probe_output.status.success() {
        return Err("ffprobe failed".into());
    }

    let dims = String::from_utf8(probe_output.stdout)?;
    let mut parts = dims.trim().split(',');

    let width: u32 = parts.next().ok_or("Missing width")?.parse()?;

    let height: u32 = parts.next().ok_or("Missing height")?.parse()?;

    let mut child = Command::new("ffmpeg")
        .args([
            "-v", "error", "-i", path, "-f", "rawvideo", "-pix_fmt", "rgb24", "-",
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut buffer = Vec::new();

    child
        .stdout
        .as_mut()
        .ok_or("Failed to capture ffmpeg stdout")?
        .read_to_end(&mut buffer)?;

    let status = child.wait()?;
    if !status.success() {
        return Err("ffmpeg decode failed".into());
    }
    let expected_size = width as usize * height as usize * 3;

    if buffer.len() != expected_size {
        return Err("Decoded buffer size mismatch".into());
    }

    Ok((buffer, width, height, PixelFormat::RGB24))
}

pub fn export_frame_to_png(
    frame: &Frame,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if frame.format() != PixelFormat::RGB24 {
        return Err("Only RGB24 export supported in V1".into());
    }

    let width = frame.width();
    let height = frame.height();

    let mut child = Command::new("ffmpeg")
        .args([
            "-v",
            "error",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgb24",
            "-s",
            &format!("{}x{}", width, height),
            "-i",
            "-",
            "-y",
            output_path,
        ])
        .stdin(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or("Failed to open ffmpeg stdin")?
        .write_all(frame.data())?;

    let status = child.wait()?;
    if !status.success() {
        return Err("ffmpeg failed to write image".into());
    }

    Ok(())
}
