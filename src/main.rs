#![allow(warnings, unused)]
use editron::experiments::audio_image_conversion::{frame_to_track, track_to_frame};
use editron::io;
use editron::io::export_frame_to_png;
use editron::io::load_image;
use editron::media::frame::{Color, Frame, FrameError, Pos};
use editron_v1 as editron;
use editron_v1::filter::Filter;
use editron_v1::filters::gaussian_blur::GaussianBlur;
use editron_v1::media::frame;
use editron_v1::media::track::Track;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "test_inputs/input.mp3";

    //println!("{frame:?}");
    Ok(())
}
