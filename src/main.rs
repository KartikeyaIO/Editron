use editron_v1::{
    io::{encode_image, load_image},
    media::frame::{Color, Pos},
};

fn main() {
    let input_path = "test_inputs/bicep.jpeg";
    let output_path = "Outputs/output.png";
    let mut frame1 = load_image(input_path, "rgba").expect("Image Loading Failed!");
    for i in (0..frame1.width()).step_by(1) {
        for j in (0..frame1.height()).step_by(1) {
            frame1.contrast(&Pos(i, j), 2.0).expect("Pixel Set Failed!");
            //frame1.brightness(&Pos(i, j), 10);
            frame1.saturation(&Pos(i, j), 2.5);
        }
    }
    encode_image(&frame1, output_path).expect("Encoding of the Image Failed!");
}
