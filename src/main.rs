#![allow(warnings, unused)]
// use editron_v1::lexer;
// use editron_v1::parser;
// use std::fs;
// use std::io;

// fn main() -> Result<(), io::Error> {
//     let source = fs::read_to_string("input.edt")?;

//     let tokens = match lexer::lexer(&source) {
//         Ok(t) => t,
//         Err(e) => {
//             eprintln!("Lexer Error: {:?}", e);
//             return Ok(());
//         }
//     };

//     for token in &tokens {
//         println!("{:?}", token);
//     }

//     let mut parser = parser::Parser::new(tokens);
//     let ir = parser.parse();

//     for instr in ir {
//         println!("{:?}", instr);
//     }

//     Ok(())
// }
use editron::export_frame_to_png;
use editron::load_image_rgb;
use editron::media::frame::{Frame, FrameError};
use editron_v1 as editron;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "image.jpg";
    let (data, w, h, fmt) = load_image_rgb(path)?;
    let mut frame = Frame::new(w, h, fmt, data)?;
    println!("{:?}", frame.format());
    //export_frame_to_png(&frame, "output.png");
    //frame.brightness(-50);
    //export_frame_to_png(&frame, "output1.png");
    Ok(())
}
