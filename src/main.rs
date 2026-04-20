use editron_v1::{
    io::{encode_image, load_image},
    media::frame::{Color, Frame, Pos},
};

fn main() {
    let input_path = "test_inputs/image.jpg";
    let output_path = "Outputs/output.png";
    let mut frame1 = load_image(input_path, "rgba").expect("Image Loading Failed!");
    let mut frame2 = load_image("test_inputs/image3.jpg", "rgba").expect("Image Loading Failed!");
    for i in (0..frame1.width()).step_by(1) {
        for j in (0..frame1.height()).step_by(1) {
            frame1
                .contrast(&Pos(i, j), 2.0)
                .expect("Contrast Increase Failed!");

            frame1.saturation(&Pos(i, j), 3.5);
        }
    }

    frame1
        .blend_on(&Pos(0, 0), &frame2, 0.50)
        .expect("Blending Failed");
    encode_image(&frame1, output_path).expect("Encoding of the Image Failed!");
}
