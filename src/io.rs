use crate::media::frame::{Frame, PixelData};
use crate::media::track::Track;
use std::io::Read;
use std::io::Write;
use std::process::{Command, Stdio};
use std::slice;

//Helper Functons

fn deinterleave_rgb(buffer: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let pixel_count = buffer.len() / 3;
    let mut r = Vec::with_capacity(pixel_count);
    let mut g = Vec::with_capacity(pixel_count);
    let mut b = Vec::with_capacity(pixel_count);
    for chunk in buffer.chunks_exact(3) {
        r.push(chunk[0]);
        g.push(chunk[1]);
        b.push(chunk[2]);
    }
    (r, g, b)
}

fn deinterleave_rgba(buffer: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
    let pixel_count = buffer.len() / 4;
    let mut r = Vec::with_capacity(pixel_count);
    let mut g = Vec::with_capacity(pixel_count);
    let mut b = Vec::with_capacity(pixel_count);
    let mut a = Vec::with_capacity(pixel_count);
    for chunk in buffer.chunks_exact(4) {
        r.push(chunk[0]);
        g.push(chunk[1]);
        b.push(chunk[2]);
        a.push(chunk[3]);
    }
    (r, g, b, a)
}

fn reinterleave_rgb(r: &[u8], g: &[u8], b: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(r.len() * 3);
    for i in 0..r.len() {
        out.push(r[i]);
        out.push(g[i]);
        out.push(b[i]);
    }
    out
}

fn reinterleave_rgba(r: &[u8], g: &[u8], b: &[u8], a: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(r.len() * 4);
    for i in 0..r.len() {
        out.push(r[i]);
        out.push(g[i]);
        out.push(b[i]);
        out.push(a[i]);
    }
    out
}

//Images

pub fn load_image(path: &str, fmt: &str) -> Result<Frame, Box<dyn std::error::Error>> {
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

    let bpp = match fmt {
        "rgb24" => 3,
        "rgba" => 4,
        "gray" => 1,
        "yuv420p" => 0,
        _ => return Err("Invalid Pixel Format".into()),
    };

    let dims = String::from_utf8(probe_output.stdout)?;
    let mut parts = dims.trim().split(',');
    let width: u32 = parts.next().ok_or("Missing width")?.parse()?;
    let height: u32 = parts.next().ok_or("Missing height")?.parse()?;

    let mut child = Command::new("ffmpeg")
        .args([
            "-v", "error", "-i", path, "-f", "rawvideo", "-pix_fmt", fmt, "-",
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

    let expected_size = match fmt {
        "yuv420p" => width as usize * height as usize * 3 / 2,
        _ => width as usize * height as usize * bpp,
    };
    if buffer.len() != expected_size {
        return Err("Decoded buffer size mismatch".into());
    }

    let data = match bpp {
        3 => {
            let (r, g, b) = deinterleave_rgb(&buffer);
            PixelData::RGB(r, g, b)
        }
        4 => {
            let (r, g, b, a) = deinterleave_rgba(&buffer);
            PixelData::RGBA(r, g, b, a)
        }
        0 => {
            let y_size = width as usize * height as usize;
            let uv_size = (width as usize / 2) * (height as usize / 2);
            let y = buffer[..y_size].to_vec();
            let u = buffer[y_size..y_size + uv_size].to_vec();
            let v = buffer[y_size + uv_size..].to_vec();
            PixelData::YUV420(y, u, v)
        }
        1 => PixelData::GRAY(buffer),
        _ => return Err("Invalied PixelFormat".into()),
    };

    Ok(Frame::new(width, height, data)?)
}

pub fn export_frame_to_png(
    frame: &Frame,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = frame.width();
    let height = frame.height();

    let raw = match frame.data() {
        PixelData::RGB(r, g, b) => reinterleave_rgb(r, g, b),
        PixelData::RGBA(r, g, b, a) => reinterleave_rgba(r, g, b, a),
        PixelData::GRAY(l) => l.clone(),
        PixelData::YUV420(y, u, v) => {
            let mut out = Vec::with_capacity(y.len() + u.len() + v.len());
            out.extend_from_slice(y);
            out.extend_from_slice(u);
            out.extend_from_slice(v);
            out
        }
    };

    let mut child = Command::new("ffmpeg")
        .args([
            "-v",
            "error",
            "-f",
            "rawvideo",
            "-pix_fmt",
            frame.data().ffmpeg_fmt(),
            "-s",
            &format!("{}x{}", width, height),
            "-i",
            "-",
            "-y",
            "-update",
            "1",
            output_path,
        ])
        .stdin(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or("Failed to open ffmpeg stdin")?
        .write_all(&raw)?;

    let status = child.wait()?;
    if !status.success() {
        return Err("ffmpeg failed to write image".into());
    }

    Ok(())
}

//Audio

pub fn decode_audio(path: &str) -> Result<(u32, u8, Vec<f32>), Box<dyn std::error::Error>> {
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
        return Err("ffmpeg audio decode failed".into());
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
        return Err("ffmpeg mp3 encoding failed".into());
    }

    Ok(())
}
