use std::io::Error;

use crate::media::{
    frame::{Frame, FrameError, PixelData},
    track::{AudioFrame, Track},
    video::TimeStamp,
};

// ── pixel → sample ────────────────────────────────────────────────────────────

#[inline]
fn byte_to_sample(b: u8) -> f32 {
    (b as f32 / 255.0) * 2.0 - 1.0
}

#[inline]
fn sample_to_byte(s: f32) -> u8 {
    ((s + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8
}

// ── Frame → Track ─────────────────────────────────────────────────────────────

/// Sonify a frame: each pixel becomes a sample, each color channel becomes an
/// audio channel. The resulting Track holds a single AudioFrame at t=0.
///
/// YUV is rejected because its channel semantics don't map cleanly to audio.
pub fn frame_to_track(frame: &Frame, sample_rate: u32) -> Result<Track, Error> {
    let channel_vecs: Vec<Vec<f32>> = match frame.data() {
        PixelData::GRAY(pixels) => {
            vec![pixels.iter().map(|&b| byte_to_sample(b)).collect()]
        }
        PixelData::RGB(r, g, b) => {
            vec![
                r.iter().map(|&b| byte_to_sample(b)).collect(),
                g.iter().map(|&b| byte_to_sample(b)).collect(),
                b.iter().map(|&b| byte_to_sample(b)).collect(),
            ]
        }
        PixelData::RGBA(r, g, b, a) => {
            vec![
                r.iter().map(|&b| byte_to_sample(b)).collect(),
                g.iter().map(|&b| byte_to_sample(b)).collect(),
                b.iter().map(|&b| byte_to_sample(b)).collect(),
                a.iter().map(|&b| byte_to_sample(b)).collect(),
            ]
        }
        _ => return Err(Error::other("frame_to_track: YUV conversion not supported")),
    };

    let channels = channel_vecs.len() as u16;
    let audio_frame = AudioFrame {
        time: TimeStamp::default(),
        data: channel_vecs,
    };

    Ok(Track::new(sample_rate, channels, vec![audio_frame]))
}

// ── Track → Frame ─────────────────────────────────────────────────────────────

/// Reconstruct a frame from a Track. Each audio channel maps back to a pixel
/// channel. Only the first AudioFrame is used — this is a single-frame
/// reconstruction, not a video decode.
///
/// The output dimensions are chosen to be the largest 16:9 rectangle that fits
/// the available sample count.
pub fn track_to_frame(track: &Track, fmt: &str) -> Result<Frame, FrameError> {
    let bpp: usize = match fmt {
        "gray" => 1,
        "rgb24" => 3,
        "rgba" => 4,
        _ => return Err(FrameError::InvalidPixelFormat),
    };

    if track.channels() as usize != bpp {
        return Err(FrameError::InvalidPixelFormat);
    }

    // Use the first AudioFrame only.
    let audio_frame = track
        .buffer()
        .first()
        .ok_or(FrameError::InvalidPixelFormat)?;

    // All channels must have the same sample count.
    let samples_per_channel = audio_frame.data.first().map_or(0, |ch| ch.len());

    if audio_frame
        .data
        .iter()
        .any(|ch| ch.len() != samples_per_channel)
    {
        return Err(FrameError::InvalidPixelFormat);
    }

    // Largest 16:9 rectangle that fits within samples_per_channel pixels.
    let k = ((samples_per_channel as f64 / 144.0).sqrt().floor() as u32).max(1);
    let width = 16 * k;
    let height = 9 * k;
    let used = (width * height) as usize;

    // Convert each channel's samples back to bytes, truncating to `used`.
    let mut chans: Vec<Vec<u8>> = audio_frame
        .data
        .iter()
        .map(|ch| {
            let mut bytes: Vec<u8> = ch.iter().map(|&s| sample_to_byte(s)).collect();
            bytes.resize(used, 0); // pad if somehow short
            bytes.truncate(used);
            bytes
        })
        .collect();

    if chans.len() < bpp {
        return Err(FrameError::InvalidPixelFormat);
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
