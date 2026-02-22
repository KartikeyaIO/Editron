use Editron::{
    filter::Filter,
    filters::gaussian_blur::GaussianBlur,
    io,
    media::{frame::Frame, track::Track},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let path = "test_files/image.jpg";
    // let (data, w, h, fmt) = io::load_image_rgb(path)?;
    // let mut frame = Frame::new(w, h, fmt, data)?;
    // //println!("{:?}", frame);
    // let blur = GaussianBlur::new(1.0);
    let d = io::decode("test_files/input.mp3")?;
    let (sr, channel, buffer) = d;
    let mut track = Track::new(sr, channel as u16, buffer);
    let peak = track.buffer().iter().fold(0.0_f32, |a, &b| a.max(b.abs()));
    println!("{track:?}");
    println!("Peak before: {}", peak);
    //track.gain(50.0);
    //io::encode_mp3(&track, "outputs/output.mp3")?;
    // for _ in 0..10 {
    //     frame = blur.apply(frame);
    // }
    // io::export_frame_to_png(&frame, "outputs/output1.png")?;
    Ok(())
}
