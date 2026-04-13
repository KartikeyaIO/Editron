use editron_v1::io::{Video, VideoEncoder, encode_image};
use editron_v1::media::video::TimeStamp;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let mut video = Video::open("test_inputs/input.mp4").expect("failed to open video");

    let tb = video.time_base();
    let time_base_num = tb.numerator() as u32;
    let time_base_den = tb.denominator() as u32;

    println!("width:     {}", video.width());
    println!("height:    {}", video.height());
    println!("time_base: {}/{}", time_base_num, time_base_den);

    let mut encoder = VideoEncoder::open(
        "Outputs/output.mp4",
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
                vf.frame.brightness(80);
                if frame_idx == 150 {
                    encode_image(&vf.frame, "Outputs/output.png").expect("Encoding Image Failed!");
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
