use media::frame::{Frame, PixelFormat};

use crate::{filter::Filter, filters::gaussian_blur::GaussianBlur};

mod engine;
pub mod experiments;
pub mod filter;
pub mod filters;
pub mod io;
pub mod lexer;
pub mod media;
pub mod parser;
pub mod text;

#[test]
fn test_image_operations() {
    let path = "test_inputs/image.jpg";
    for i in vec![PixelFormat::RGB24, PixelFormat::RGBA32, PixelFormat::Gray8] {
        let (data, w, h, fmt) =
            io::load_image(path, i).expect("The Load Failed! check Image or functions!");
        let frame = match Frame::new(w, h, fmt, data) {
            Ok(value) => value,
            Err(e) => {
                panic!("Frame Creation Failed!{e}");
            }
        };

        let output_path = format!("Outputs/output{}.png", i.ffmpeg_fmt());

        match io::export_frame_to_png(&frame, &output_path) {
            Ok(_) => println!("The Test was successful! and Output is stored at: {output_path}"),
            Err(_) => panic!("Export to PNG Failed"),
        }
    }
}
#[test]
fn blur_effect_test() {
    let path = "test_inputs/image.jpg";
    let blur = GaussianBlur::new(6.0);
    for i in vec![PixelFormat::RGB24, PixelFormat::RGBA32, PixelFormat::Gray8] {
        let (data, w, h, fmt) =
            io::load_image(path, i).expect("The Load Failed! check Image or functions!");
        let mut frame = match Frame::new(w, h, fmt, data) {
            Ok(value) => value,
            Err(e) => {
                panic!("Frame Creation Failed!{e}");
            }
        };
        frame = blur.apply(frame);

        let output_path = format!("Outputs/output_blur{}.png", i.ffmpeg_fmt());

        match io::export_frame_to_png(&frame, &output_path) {
            Ok(_) => println!("The Test was successful! and Output is stored at: {output_path}"),
            Err(_) => panic!("Export to PNG Failed"),
        }
    }
}
