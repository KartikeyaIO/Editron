use std::io::Error;

use crate::media::{
    frame::{Frame, FrameError, PixelData},
    track::Track,
};

pub fn frame_to_track(f: &Frame, channels: u16) -> Result<Track, Error> {
    let data = f.data();

    let mut buffer = Vec::with_capacity(data.len());
    match data {
        PixelData::GRAY(pixels) => {
            for &pixel in pixels {
                let sample = (pixel as f32 / 255.0) * 2.0 - 1.0;
                buffer.push(sample);
            }
        }
        PixelData::RGB(r, g, b) => {
            for pixel in 0..r.len() {
                let sample = (r[pixel] as f32 / 255.0) * 2.0 - 1.0;
                let sample1 = (g[pixel] as f32 / 255.0) * 2.0 - 1.0;
                let sample2 = (b[pixel] as f32 / 255.0) * 2.0 - 1.0;
                buffer.push(sample);
                buffer.push(sample1);
                buffer.push(sample2);
            }
        }
        PixelData::RGBA(r, g, b, a) => {
            for pixel in 0..r.len() {
                let sample = (r[pixel] as f32 / 255.0) * 2.0 - 1.0;
                let sample1 = (g[pixel] as f32 / 255.0) * 2.0 - 1.0;
                let sample2 = (b[pixel] as f32 / 255.0) * 2.0 - 1.0;
                let sample3 = (a[pixel] as f32 / 255.0) * 2.0 - 1.0;
                buffer.push(sample);
                buffer.push(sample1);
                buffer.push(sample2);
                buffer.push(sample3);
            }
        }
        _ => return Err(Error::other("Conversion is not possible for YUV")),
    }

    Ok(Track::new(44100, channels, buffer))
}

pub fn track_to_frame(track: &Track, fmt: &str) -> Result<Frame, FrameError> {
    let pixels: Vec<u8> = track
        .buffer()
        .iter()
        .map(|&s| ((s + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8)
        .collect();

    let bpp = match fmt {
        "gray" => 1,
        "rgb24" => 3,
        "rgba" => 4,
        _ => return Err(FrameError::InvalidPixelFormat),
    };

    if pixels.len() % bpp != 0 {
        return Err(FrameError::InvalidPixelFormat);
    }

    let total_pixels = pixels.len() / bpp;
    let k = ((total_pixels as f64) / 144.0).sqrt().floor() as u32;
    let width = 16 * k;
    let height = 9 * k;
    let used_pixels = (width * height) as usize;

    let mut chans: Vec<Vec<u8>> = vec![Vec::with_capacity(used_pixels); bpp];
    for (i, &val) in pixels.iter().enumerate().take(used_pixels * bpp) {
        chans[i % bpp].push(val);
    }
    for ch in &mut chans {
        ch.resize(used_pixels, 0);
    }

    let pixel_data = match fmt {
        "gray" => PixelData::GRAY(chans.remove(0)),
        "rgb24" => PixelData::RGB(chans.remove(0), chans.remove(0), chans.remove(0)),
        "rgba" => PixelData::RGBA(
            chans.remove(0),
            chans.remove(0),
            chans.remove(0),
            chans.remove(0),
        ),
        _ => return Err(FrameError::InvalidPixelFormat),
    };

    Frame::new(width, height, pixel_data)
}
