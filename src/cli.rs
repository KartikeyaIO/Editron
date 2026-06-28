use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::engine::Engine;
use crate::parser;
use crate::media::frame::{Frame, PixelData};
use crate::io::io::encode_image;

#[derive(Parser)]
#[command(name = "drive")]
#[command(version, about = "DriveLang CLI - A cool media procesing Domain Specific language", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    
    #[arg(short, long, global = true)]
    pub time: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    
    New {
        
        name: String,
    },
    
    Run {
        
        file: Option<String>,
    },
}

pub fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
   
    let cli = Cli::parse();
    
    
    let start = Instant::now();

    match &cli.command {
        Commands::New { name } => {
            create_project(name)?;
            println!("Created new Drive project: {}", name);
            println!("Run it using: cd {} && drive run", name);
        }
        Commands::Run { file } => {
            
            let file_path = file.clone().unwrap_or_else(|| "main.drive".to_string());
            
            if !Path::new(&file_path).exists() {
                return Err(format!("File '{}' not found. Did you forget where you parked your project?", file_path).into());
            }

            let source = fs::read_to_string(&file_path)?;
            let program = parser::parse(&source).map_err(|e| format!("Parsing Failed: {:?}", e))?;
            
            let mut engine = Engine::new();
            engine.run(&program).map_err(|e| format!("Error: {:?}", e))?;
        }
    }

    
    if cli.time {
        println!("Execution completed in: {:?}", start.elapsed());
    }

    Ok(())
}

fn create_project(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(name);
    
   
    fs::create_dir_all(base_path)?;
    fs::create_dir_all(base_path.join("assets"))?;
    fs::create_dir_all(base_path.join("output"))?;

    
    let dummy_width = 400;
    let dummy_height = 100;
    let len = dummy_width * dummy_height;
    
    let pixel_data = PixelData::RGBA(
        vec![255; len], 
        vec![200; len], 
        vec![200; len], // B
        vec![255; len], // A
    );
    
    if let Ok(frame) = Frame::new(dummy_width as u32, dummy_height as u32, pixel_data) {
        let asset_path = base_path.join("assets/drivelang.png");
        let _ = encode_image(&frame, asset_path.to_str().unwrap());
    }

    // The beautiful boilerplate
    let boilerplate = r#"
canvas = blank(800, 600);


filter stylish() {
    r = (x / width) * 25.0;
    g = (y / height) * 25.0;
    b = 150.0;
    a = 255.0;
}


background = canvas -> stylish();
drivelang = frame("assets/drivelang.png");
final_img = background -> blend(200, 250, drivelang, 0.9);
export(final_img, "output/result.png");
print("Successfully rendered to output/result.png ");
"#;

    fs::write(base_path.join("main.drive"), boilerplate)?;
    
    Ok(())
}