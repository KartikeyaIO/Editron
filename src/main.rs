use editron_v1::{
    filter::Filter,
    filters::gaussian_blur,
    io::{Video, VideoEncoder, encode_image, load_image},
    media::frame::{Frame, Pos},
};

use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let mut frame1 = load_image("test_inputs/image.jpg", "rgba").expect("Image Loading Failed!");

    let mut video = Video::open("test_inputs/input2.mp4").expect("failed to open video");

    let tb = video.time_base();
    let time_base_num = tb.numerator() as u32;
    let time_base_den = tb.denominator() as u32;

    println!("width:     {}", video.width());
    println!("height:    {}", video.height());
    println!("time_base: {}/{}", time_base_num, time_base_den);

    let mut encoder = VideoEncoder::open(
        "Outputs/output1.mp4",
        video.width(),
        video.height(),
        tb,
        ffmpeg_next::Rational::new(30, 1),
    )
    .expect("failed to open encoder");

    let mut frame_count = 0u32;
    let mut frame_idx = 0i64;

    loop {
        match video.decode_next() {
            Ok(Some(mut vf)) => {
                (frame1, vf.frame) =
                    Frame::normalize(&frame1, &vf.frame).expect("Normalization Failed!");

                vf.frame.blend(&frame1, 0.65).expect("Blending Failed");
                for i in (0..vf.frame.width()).step_by(7) {
                    for j in (0..vf.frame.height()).step_by(2) {
                        vf.frame.brightness(&Pos(i, j), 80);
                        vf.frame
                            .contrast(&Pos(i, j), 2.5)
                            .expect("Constrast Increase Failed!");
                    }
                }
                encoder.encode_frame(&vf, frame_idx).expect("encode failed");
                frame_count += 1;
                frame_idx += 1;
            }
            Ok(None) => {
                println!("EOF reached");
                break;
            }
            Err(e) => {
                eprintln!("error: {:?}", e);
                break;
            }
        }
    }

    encoder.finish().expect("failed to finalize");

    let elapsed = start_time.elapsed();
    println!("frames processed : {}", frame_count);
    println!("time taken        : {:.2?}", elapsed);
    println!("avg per frame     : {:.2?}", elapsed / frame_count.max(1));
}
