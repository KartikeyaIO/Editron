mod lexer;
use lexer::lexer;

fn main() {
    let source = r#"
load video = "input.mp4";
load img = "frame.png";
video[0..30].brightness(50);
video[0, 3, 7].contrast(-20);
img[0].blur(1.5);
filter darken {
    video[0].brightness(-10);
    video[1].contrast(30);
}
export video "output.mp4";
"#;

    match lexer(source) {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
