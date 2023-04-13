mod compressor;

use std::path::Path;
use clap::{command, Parser};
use crate::compressor::Compressor;

/// Image Compresser Application
#[derive(Parser, Debug)]
#[command(name = "ic", about = "Image Compresser Application", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// Image File Path
    #[arg(short, long)]
    image: String,
}

fn main() {
    let img_path = Cli::parse().image;
    println!("{img_path:?}");

    if !Path::new(&img_path).exists() {
        eprintln!("Invalid Path: `{img_path}`");
        panic!("Invalid Path");
    }

    let engine = Compressor::new();

    if let Ok(_) = engine.compress(&img_path) {
        println!("Compression Finished");
    } else {
        println!("Compression Failed")
    }
}