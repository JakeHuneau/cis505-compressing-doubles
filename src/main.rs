//use compressing_doubles::{CompressionMode, Mode};
//use gorilla::decode::gorilla_decode;
//use gorilla::encode::gorilla_encode;
//use std::env;
//use std::process::exit;
use std::io::Cursor;
use std::vec::Vec;

use sprintz::sprintz_decoder::SprintzDecoder;
mod sprintz;

fn main() {
    let input = Cursor::new(Vec::new());
    let decoder = SprintzDecoder::new(input,0);
    
    
}
/*
mod gorilla;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Must have 4 arguments: <gorilla/spritz> <encode/decode> <input_filename> <output_filename>");
        exit(1);
    }
    let compression_mode: CompressionMode;
    let mode: Mode;

    // Check first input for compression mode
    match args[1].to_lowercase().as_str() {
        "gorilla" | "g" => {
            compression_mode = CompressionMode::Gorilla;
        }
        "sprintz" | "s" => {
            compression_mode = CompressionMode::Sprintz;
        }
        _ => {
            println!("Compression mode must be gorilla or sprintz.");
            exit(1);
        }
    }

    // Check second input for compression mode
    match args[2].to_lowercase().as_str() {
        "encode" | "e" => {
            mode = Mode::Encode;
        }
        "decode" | "d" => {
            mode = Mode::Decode;
        }
        _ => {
            println!("Must be either encode or decode mode.");
            exit(1);
        }
    }

    let input_filename = &args[3];
    let output_filename = &args[4];

    if compression_mode == CompressionMode::Gorilla {
        if mode == Mode::Encode {
            gorilla_encode(input_filename, output_filename);
        } else {
            gorilla_decode(input_filename, output_filename);
        }
    } else {
        println!("You chose Sprintz mode")
    }
    
}*/
