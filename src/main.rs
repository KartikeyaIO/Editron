#![allow(warnings, unused)]
use editron::io;
use editron::io::export_frame_to_png;
use editron::io::load_image_rgb;
use editron::media::frame::{Color, Frame, FrameError, Pos};
use editron_v1 as editron;
use editron_v1::filter::Filter;
use editron_v1::filters::gaussian_blur::GaussianBlur;
use editron_v1::media::frame;
use editron_v1::media::track::Track;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "test_inputs/input.mp3";
    let (sr, channel, buffer) = io::decode(path)?;
    let mut track = Track::new(sr, channel as u16, buffer);
    track.gain(-10.6);

    //println!("{frame:?}");
    Ok(())
}
