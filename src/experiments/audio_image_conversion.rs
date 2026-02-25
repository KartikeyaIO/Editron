use std::io::Error;

use crate::media::{
    frame::{Frame, FrameError},
    track::Track,
};

pub fn frame_to_track(f: &Frame, channels: u16) -> Result<Track, Error> {
    let data = f.data();

    let mut buffer = Vec::with_capacity(data.len());

    for &pixel in data {
        let sample = (pixel as f32 / 255.0) * 2.0 - 1.0;
        buffer.push(sample);
    }

    Ok(Track::new(44100, channels, buffer))
}

pub fn track_to_frame(track: &Track) -> Result<Frame, FrameError> {
    let buffer = track.buffer();
    let mut data = Vec::with_capacity(buffer.len());

    for &sample in buffer {
        let pixel = ((sample + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;
        data.push(pixel);
    }

    let total_pixels = data.len() / 3;
    let k = ((total_pixels as f64) / 144.0).sqrt().floor() as u32;

    let width = 16 * k;
    let height = 9 * k;

    let used_pixels = (width * height) as usize;
    let used_bytes = used_pixels * 3;

    if data.len() > used_bytes {
        data.truncate(used_bytes);
    } else if data.len() < used_bytes {
        data.resize(used_bytes, 0);
    }

    Frame::new(width, height, crate::media::frame::PixelFormat::RGB24, data)
}
