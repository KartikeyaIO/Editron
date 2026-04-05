use editron_v1::{
    io::{convert_to_reel, encode_image, load_image},
    media::frame::Pos,
};
fn main() {
    let output_path = "Outputs/output.reel";
    let input_path = "test_inputs/input.mp4";
    convert_to_reel(input_path, output_path).expect("Conversion Failed!");
}
