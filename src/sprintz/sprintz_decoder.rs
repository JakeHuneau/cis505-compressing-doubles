use std::io;
use std::io::Read;
use super::forecaster::Forecaster;


pub struct SprintzDecoder<'a>
{
    input: SprintzInput<'a>,
    block_size: u32,
    forecaster: Forecaster,
    read_pos: u32,
    left_in_block: u32,
    zeroes_left: u64,
    nbits: u32,
    
}


 impl SprintzDecoder<'_> {
    pub fn new<'a>(datainput: &'a mut dyn Read, block_size: u32)-> SprintzDecoder<'a>
    {
         SprintzDecoder 
         {
            input: SprintzInput::new(datainput),
            block_size,
            forecaster: Forecaster::new(),
            read_pos: 0,
            left_in_block: 0,
            zeroes_left: 0,
            nbits: 0,
         }
         
    }
    
    
    pub fn read_value(&mut self) -> io::Result<f64>
    {
        let data: u64 =self.read_value_raw()?;
        
        Ok(f64::from_bits(data))
     
    }
   pub fn read_value_raw(&mut self) -> io::Result<u64>
    {
        if self.left_in_block > 0 {
            let mut xor: u64 = self.get_bits(1)? << 63;
            xor |= self.get_bits(self.nbits)?;
            self.left_in_block-=1;
            
            let ret = self.forecaster.error(xor);
            //println!("Decode {} -> {} -> {}",self.forecaster.predict(), xor, ret);
            self.forecaster.train(ret, xor);
           
            return Ok(ret);
        } else if self.zeroes_left > 0 {
            //println!("Encode zero {}", self.zeroes_left);
            self.zeroes_left-=1;
     
            return Ok(self.forecaster.predict());
        } else {
            self.nbits = self.get_bits(7)? as u32;
            //println!("Decode nbits {}",   self.nbits);
            if self. nbits == 0 {
                let num_zero_blocks = self.get_bits(16)?;
                self.zeroes_left = num_zero_blocks * (self.block_size as u64);
                //println!("Decode zero {}", self.zeroes_left);
            } else {
                //println!("Decode open block");
                self.left_in_block = self.block_size;
            }
            
            return self.read_value_raw();
        }
        
    }
    
    fn get_bits(&mut self, bits: u32) -> io::Result<u64> {
        self.read_pos += bits;
        return self.input.read_long(bits);
    }
}


struct SprintzInput<'a> {
    input: &'a mut  dyn Read,
    bits_left: u32,
    byte_buffer: u8
    
}

impl SprintzInput<'_> {
    
    fn new<'a>(input: &'a  mut dyn Read) -> SprintzInput<'a>{
        SprintzInput{
            input,
            bits_left: 0,
            byte_buffer: 0
        }
    }
    
    fn buffer_byte(&mut self) -> io::Result<()>{
        if self.bits_left == 0 {
            let mut buffer:[u8;1] = [0;1];
            self.input.read_exact(&mut buffer)? ;
            self.byte_buffer = buffer[0];
            //println!("Decode bits {}",self.byte_buffer );
            self.bits_left = 8;
        }
        
        return Ok(());
    }
    
    fn read_long(&mut self, mut bits:u32) -> io::Result<u64> {
        let mut value = 0u64;
        while bits > 0 {
            let buffer :u64 = self.byte_buffer.into();
            if bits > self.bits_left || bits == 8 {
                // Take only the bits_left "least significant" bits
                let d: u64 = buffer  & ((1u64 << self.bits_left) - 1);
                value = (value << self.bits_left) + (d & 0xFFu64);
                bits -= self.bits_left;
                self.bits_left = 0;
            } else {
                // Shift to correct position and take only least significant bits
                let d: u64 =  (buffer >> (self.bits_left - bits)) & ((1u64<<bits) - 1);
                value = (value << bits) + (d & 0xFFu64);
                self.bits_left -= bits;
                bits = 0;
            }
            self.buffer_byte()?;
        }
        return Ok(value);
    }
}