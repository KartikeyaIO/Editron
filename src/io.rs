use crate::media::frame::{Frame, PixelFormat};
use crate::media::track::Track;
use std::io::Read;
use std::io::Write;
use std::process::{Command, Stdio};
use std::slice;
pub fn load_image(
    path: &str,
    fmt: PixelFormat,
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
            "-v",
            "error",
            "-i",
            path,
            "-f",
            "rawvideo",
            "-pix_fmt",
            fmt.ffmpeg_fmt(),
            "-",
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
    let expected_size = width as usize * height as usize * fmt.bytes_per_pixel();

    if buffer.len() != expected_size {
        return Err("Decoded buffer size mismatch".into());
    }

    Ok((buffer, width, height, fmt))
}

pub fn export_frame_to_png(
    frame: &Frame,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = frame.width();
    let height = frame.height();

    let mut child = Command::new("ffmpeg")
        .args([
            "-v",
            "error",
            "-f",
            "rawvideo",
            "-pix_fmt",
            frame.format().ffmpeg_fmt(),
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

pub fn decode(path: &str) -> Result<(u32, u8, Vec<f32>), Box<dyn std::error::Error>> {
    let mut child = Command::new("ffmpeg")
        .args([
            "-i",
            path,
            "-f",
            "f32le",
            "-acodec",
            "pcm_f32le",
            "-ac",
            "2",
            "-ar",
            "44100",
            "pipe:1",
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut raw_output = Vec::new();
    child
        .stdout
        .as_mut()
        .unwrap()
        .read_to_end(&mut raw_output)?;
    let status = child.wait()?;
    if !status.success() {
        return Err("FFMPEG Failed".into());
    }
    let samples: Vec<f32> = raw_output
        .chunks_exact(4)
        .map(|bytes| f32::from_le_bytes(bytes.try_into().unwrap()))
        .collect();

    Ok((44100, 2, samples))
}

pub fn encode_mp3(track: &Track, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("ffmpeg")
        .args([
            "-y",
            "-f",
            "f32le",
            "-ac",
            &track.channels().to_string(),
            "-ar",
            &track.sample_rate().to_string(),
            "-i",
            "pipe:0",
            "-vn",
            "-c:a",
            "libmp3lame",
            output,
        ])
        .stdin(Stdio::piped())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().unwrap();

        let samples = track.buffer();
        let byte_slice: &[u8] = unsafe {
            slice::from_raw_parts(
                samples.as_ptr() as *const u8,
                samples.len() * std::mem::size_of::<f32>(),
            )
        };

        stdin.write_all(byte_slice)?;
    }

    let status = child.wait()?;
    if !status.success() {
        return Err("ffmpeg encoding failed".into());
    }

    Ok(())
}
