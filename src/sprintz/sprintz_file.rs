use std::fs::File;
use std::io::Write;
use std::fs;
use super::sprintz_encoder::SprintzEncoder;
use super::sprintz_decoder::SprintzDecoder;

const block_size: u32 = 8;

pub fn sprintz_decode(input_filename: &str, output_filename: &str) {
    // Initialize reader
    let mut file = File::open(input_filename).expect("Could not open file");
    let mut f = File::create(output_filename).expect("Could not create output file");

    let mut values: Vec<f64> = Vec::new();
    let mut decode = SprintzDecoder::new(&mut file, block_size);
;
    
   loop {
        let mut value = decode.read_value();
        match value {
            Ok(data) => {
                values.push(data);
                //println!("Decode value {}", data);
                // break;
            }
            _ => {
                //println!("Encountered an error decoding file");
                break;
            }
        }
  
    }
     

    //println!("Decode {} lines", values.len());
    let value_strings: Vec<String> = values.iter().map(|&n| n.to_string()).collect();
    writeln!(f, "{}", value_strings.join(",")).unwrap();
    
}

pub fn sprintz_encode(input_filename: &str, output_filename: &str) {
    let raw_data = fs::read_to_string(input_filename).expect("Unable to read file");
    let data = raw_data
        .replace("\n", "")
        .split(",")
        .filter_map(|s| s.parse::<f64>().ok())
        .collect::<Vec<f64>>();

    // Start by making a buffer that writes into a file called input_filename
    let mut f = File::create(output_filename).expect("Could not create file");
    let mut decode = SprintzEncoder::new(&mut f, block_size);
    
    for value in &data {
        let res = decode.write(*value);
        
        if let Err(_) = res {
            println!("Encountered an error encoding the file");
            break;
        }
       
    }
    
    decode.flush();
    //println!("Encoded {} lines", data.len());
    
    
    
    
}