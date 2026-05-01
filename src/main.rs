use editron_v1::{
    io::{encode_image, load_image},
    media::frame::{Color, Frame, Pos},
};

fn main() {
    let input_path = "test_inputs/freshers2.jpeg";
    let output_path = "Outputs/output.png";
    let mut frame1 = load_image(input_path, "rgba").expect("Image Loading Failed!");

    for i in (0..frame1.width()).step_by(1) {
        for j in (0..frame1.height()).step_by(1) {
            frame1
                .contrast(&Pos(i, j), 2.0)
                .expect("Contrast Increase Failed!");

            frame1.saturation(&Pos(i, j), 3.5);
        }
    }

    encode_image(&frame1, output_path).expect("Encoding of the Image Failed!");
}
