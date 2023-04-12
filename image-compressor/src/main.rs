mod image;
extern crate clap;

use clap::{command, Parser};
use image::{compress_file, Options};
use core::panic;
use std::path::Path;

/// Image Compresser Application
#[derive(Parser, Debug)]
#[command(name = "ic", about = "Image Compresser Application", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// Image File Path 
    #[arg(short, long)]
    image: String,

    /// Add Extention Flag
    #[arg(short = 'o', long = "option", default_value_t = true)]
    add_ext: bool,
}


fn main() {

    let mut opt: Options = Options { add_ext:true };

    let img_path = Cli::parse().image;
    let op = Cli::parse().add_ext;

    println!("{}{}", img_path, op);

    if !Path::new(&img_path).exists() {
        eprintln!("Invalid Path: `{img_path}`");
        panic!("Invalid Path");
    }

    if !op {
        opt.add_ext = false;
    }

    compress_file(img_path, opt);
}
