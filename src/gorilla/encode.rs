use bit_streamer::Writer;
use std::fs;
use std::fs::File;

pub fn gorilla_encode(input_filename: &str, output_filename: &str) {
    let raw_data = fs::read_to_string(input_filename).expect("Unable to read file");
    let data = raw_data
        .replace("\n", "")
        .split(",")
        .filter_map(|s| s.parse::<f64>().ok())
        .collect::<Vec<f64>>();

    // Start by making a buffer that writes into a file called input_filename.gorilla
    let mut writer = Writer::new(File::create(output_filename).expect("Could not create file"));

    // First write the first value in full
    let mut previous_value = data[0].to_bits();
    // println!("value: ({:#15}) = {:#066b}", data[0], previous_value);

    // Initialize leading and trailing zeros
    let mut previous_leading_zeros: u32 = 64;
    let mut previous_trailing_zeros: u32 = 64;

    writer.write_bits(previous_value as u128, 64).unwrap();
    for &d in &data[1..] {
        let next_value = d.to_bits();
        let xor = next_value ^ previous_value;

        // Update previous values for next iteration
        previous_value = next_value;

        if xor == 0 {
            // If there is no difference from previous value, then we write a 0
            writer.write_bit(false).unwrap();
        // println!("value: ({:#15}) - Same value found, writing 0", d);
        } else {
            // Otherwise a 1, followed by more logic to show difference
            writer.write_bit(true).unwrap();

            let current_leading_zeros = xor.leading_zeros();
            let current_trailing_zeros = xor.trailing_zeros();

            // println!(
            //     "value: ({:#015}) : {:#066b} | xor : ({:017}) = {:#066b} | leading zeros = {} | prev leading zeros = {} | trailing zeros = {} | prev trailing zeros = {}",
            //     d,
            //     d.to_bits(),
            //     xor,
            //     xor,
            //     current_leading_zeros,
            //     previous_leading_zeros,
            //     current_trailing_zeros,
            //     previous_trailing_zeros
            // );

            // If block of meaningful bits is within previous meaningful bits
            if current_leading_zeros >= previous_leading_zeros
                && current_trailing_zeros >= previous_trailing_zeros
            {
                // Write a zero control bit followed by meaningful bits
                writer.write_bit(false).unwrap();
                writer
                    .write_bits(
                        xor.wrapping_shr(previous_trailing_zeros) as u128,
                        (64 - previous_leading_zeros - previous_trailing_zeros) as usize,
                    )
                    .unwrap();
            } else {
                // Otherwise, we write a 1 control bit, followed by the 5 bits of the number of
                // leading zeros, then 6 bits of the number of significant bits
                // Followed by the significant bits
                writer.write_bit(true).unwrap();
                writer.write_bits(current_leading_zeros as u128, 5).unwrap();

                let significant_bits = 64 - current_leading_zeros - current_trailing_zeros;
                writer
                    .write_bits((significant_bits - 1) as u128, 6)
                    .unwrap();
                writer
                    .write_bits(
                        xor.wrapping_shr(current_trailing_zeros) as u128,
                        significant_bits as usize,
                    )
                    .unwrap();
                previous_trailing_zeros = current_trailing_zeros;
                previous_leading_zeros = current_leading_zeros;
            }
        }
    }

    // Write an end marker that says it's a new value, with more meaningful bits, 0 leading zeros,
    // 64 significant values, then a total value of 0. This cannot happen so it's a safe end marker
    writer
        .write_bits(0b11_00000_111111 << (128 - 13), 128)
        .unwrap();
    writer.flush().unwrap();
}
