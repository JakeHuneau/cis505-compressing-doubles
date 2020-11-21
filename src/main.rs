mod sprintz;
use sprintz::SprintzCompressor;

fn main() {
    println!("Hello, world!");
    
    let data: [f64;5] = [1.0, 2.0, 1.0, 3.0, 5.0];
    
    let sprz = SprintzCompressor::new(&data, 10);
}
