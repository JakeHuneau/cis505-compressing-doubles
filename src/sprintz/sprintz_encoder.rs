use std::io::Write;
use std::vec::Vec;
use std::io;
use super::forecaster::Forecaster;
use std::convert::TryInto;

pub struct SprintzEncoder<'a>
{
    output: SprintzOutput<'a>,
    block_size: u32,
    forecaster: Forecaster,
    block: Vec<u64>,
    block_pos: usize,
    zero_blocks: u16
    
}


 impl SprintzEncoder<'_> {
    pub fn new<'a>(data_output: &'a mut dyn Write, block_size: u32)-> SprintzEncoder<'a>
    {
        SprintzEncoder 
         {
            output: SprintzOutput::new(data_output),
            block_size,
            forecaster: Forecaster::new(),
            block: Vec::with_capacity(block_size.try_into().unwrap()),//u32 into usize 
            block_pos: 0, 
            zero_blocks: 0,
         }
    }
    
    pub fn write(&mut self, value: f64) -> io::Result<()> {
        self.write_raw(value.to_bits())
    } 
    
    pub fn write_raw(&mut self, value: u64) -> io::Result<()> {
        let pos: u32 = self.block_pos.try_into().unwrap();
        if  pos == self.block_size {
            self.block_pos = 0;
            self.compress_block(false)?;
        }
        
        let error: u64 = self.forecaster.error(value);
        self.forecaster.train(value, error);    
        self.block[self.block_pos] = error;
        self.block_pos+=1;
            
        Ok(())
    
    }
    
    pub fn flush(&mut self) -> io::Result<()> {
        
        self.block_pos+=1;
        self.compress_block(true)?;
        
        Ok(())
    }
    
    
    fn compress_block (&mut self, flushing: bool) -> io::Result<()> {
        let mut b = self.block[0];
        for i in 1..self.block_size {
            let index: usize = i.try_into().unwrap();
            b |= self.block[index];
        }
        
        let nbits: u64 = leading_zeroes(b);
        
        if nbits == 0 && get_bit(b,64) == false {
            self.zero_blocks+=1;
            if flushing {
                self.output.write_bits(0, 7)?;
                self.output.write_bits(self.zero_blocks as u64, 16)?;
            }
        } else {
            if self.zero_blocks > 0 {
                self.output.write_bits(0, 7)?;
                self.output.write_bits(self.zero_blocks as u64, 16)?;
                self.zero_blocks = 0;
            }
            
            self.output.write_bits(nbits, 7)?;
            let num_to_add = if self.block_pos == 0 { self.block_size} else { self.block_pos.try_into().unwrap()};
            for i in 0..num_to_add {
            
                let index : usize = i.try_into().unwrap();
                let err = self.block[index];
                let err_bit = if get_bit(err,63) { 1 } else {0};
                
                self.output.write_bits(err_bit,1)?;
                self.output.write_bits(err, nbits as u32)?;
            }
        }
        self.block.clear();
        Ok(())
    }
    
    
}

fn leading_zeroes(data: u64) -> u64 {
    for i in (0..63).rev() {
        if get_bit(data, i) {
            return (i + 1) as u64;
        }
    }
    return 0u64;
}
    
    
fn get_bit(value: u64, bit: u32) -> bool{
    return (value >> bit) & 1 == 1;
}

struct SprintzOutput<'a> {
    output: &'a mut dyn Write,
    bits_left: u32,
    byte_buffer: u8
    
}

impl SprintzOutput<'_> {
    
    fn new<'a>(output: &'a  mut dyn Write) -> SprintzOutput<'a>
    {
        SprintzOutput{
            output,
            bits_left: 0,
            byte_buffer: 0
        }
    }
    
    fn buffer_byte(&mut self) -> io::Result<()>
    {
        if self.bits_left == 0 {
            let mut buffer:[u8;1] = [self.byte_buffer];
           
            self.output.write_all(&mut buffer)? ;
            self.byte_buffer = 0;
            self.bits_left = 8;
        }
        
       Ok(())
    }
    
    fn write_bits(&mut self, value: u64, mut bits:u32) ->  io::Result<()>
    {
        while bits > 0 {
            let mut shift = bits as i32 - self.bits_left as i32;
            if shift >= 0 {
                self.byte_buffer |=  ((value >> shift) & ((1 << self.bits_left) - 1)) as u8;
                bits -= self.bits_left;
                self.bits_left = 0;
            } else {
                shift = self.bits_left as i32 - bits as i32;
                self.byte_buffer  |=  (value << shift) as u8;
                self.bits_left -= bits;
                bits = 0;
            }
            self.buffer_byte()?;
        }
        
        Ok(())
    }
}