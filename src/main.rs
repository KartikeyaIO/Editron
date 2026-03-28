use editron_v1::io::{convert_to_reel, encode_image, load_image};
fn main() {
    let path = "Outputs/video_test.reel";
    println!("Converting to reel");
    convert_to_reel("test_inputs/input.mp4", path).expect("The Conversion Failed!");
    println!("Conversion complete file is saved at : {path}");
}
