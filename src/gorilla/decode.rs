use bit_streamer::Reader;
use std::convert::TryInto;
use std::fs::File;
use std::io::Write;

pub fn gorilla_decode(input_filename: &str, output_filename: &str) {
    // Initialize
    let mut values: Vec<f64> = Vec::new();
    let mut leading_zeros: u128 = 0;
    let mut trailing_zeros: u128 = 0;

    // Initialize reader
    let file = File::open(input_filename).expect("Could not open file");
    let mut reader = Reader::new(file);

    // Read the first value
    let mut previous_value = u64::from_be_bytes(
        reader
            .read_bytes(8)
            .unwrap()
            .try_into()
            .expect("Read wrong number of bytes"),
    );
    values.push(f64::from_bits(previous_value));
    // println!("first value: {}", f64::from_bits(previous_value));

    // Now loop through entire file
    loop {
        // println!("Looping...");
        let next_bit: bool;
        let eof_check = reader.read_bit();
        if eof_check.is_err() {
            // Break when we don't read anymore bits
            break;
        } else {
            next_bit = eof_check.unwrap();
        }
        if !next_bit {
            // println!("Read 0: Same value");
            // If next bit is 0, then it's the same value as previously
            values.push(f64::from_bits(previous_value));
        } else {
            // next bit was 1 and there's a difference from last bit
            if reader.read_bit().unwrap() {
                // If control bit is 1, we get number of leading zeros from next 5 bits
                // then the length of meaningful XORed value in the next 6 bits
                leading_zeros = reader.read_bits(5).unwrap();
                trailing_zeros = 64 - leading_zeros - (reader.read_bits(6).unwrap() + 1);
                // println!(
                //     "Control bit was 1. {} leading zeros and {} trailing zeros",
                //     leading_zeros, trailing_zeros
                // );
            }
            let size = 64 - leading_zeros - trailing_zeros;
            let next_bits = reader.read_bits(size as usize).unwrap() as u64;

            // Check for end marker
            if leading_zeros == 0 && size == 64 && next_bits == 0 {
                break;
            }

            previous_value ^= next_bits << trailing_zeros;
            // println!(
            //     "Size should be {} and read {} to get new value {}",
            //     size, next_bits, previous_value,
            // );
            values.push(f64::from_bits(previous_value));
        }
        // println!("{}", f64::from_bits(previous_value));
    }
    // println!("{:?}", &values);
    let mut f = File::create(output_filename).expect("Could not create output file");
    let value_strings: Vec<String> = values.iter().map(|&n| n.to_string()).collect();
    writeln!(f, "{}", value_strings.join(",")).unwrap();
}
