//mod sprintz;

use std::io::Cursor;
use std::io::Write;
use std::vec::Vec;

pub struct Forecaster {
    previous: f64,
    
    
}

impl Forecaster {
    
    fn predict(&self, value: f64) -> f64 {
        self.previous
    }
    
    
    fn error(&self, value: f64) -> f64 {
        //Change later to bitwise XOR
        value - self.predict(value)
    }
    
    
    //Trains the forecaster for a better prediction
    //Out algorithm just returns the previous value
    fn train(&mut self, value:  f64, error: f64) {
        self.previous = value;
    }
}


pub struct SprintzCompressor<'a>{
    input: &'a[f64],
    block: Vec<f64>,
    
    blockPos: usize,
    blockSize : usize,
    forecaster: Forecaster,
    
    
}

 impl SprintzCompressor<'_> {
    
    pub fn new<'a>(input: &'a[f64], blockSize: usize)-> SprintzCompressor{
         SprintzCompressor {
             input,
             block: Vec::with_capacity(blockSize),//u32 into usize 
             blockPos: 0, 
             blockSize: blockSize,
             forecaster: Forecaster{ previous: 0.0}
         }
         
        
    }
    
    fn compress<'a>(&mut self, output: &mut Cursor<Vec<u8>>) 

    {
        let len: u32 = output.get_ref().len() as u32;
        
        //Break down integer
        let lenBytes: [u8; 4] = [
            ((len >> 24) & 0xFF ) as u8, 
             ((len >> 16) & 0xFF) as u8, 
             ((len >> 8) & 0xFF) as u8, 
             (len & 0xFF) as u8
             ];

        output.write(&lenBytes);
        
        for val in self.input 
        {
            
           let compress = self.submitDouble(*val);
           
           if compress {
            //Compress
             } 
        }
        
        //flush(); //Not implemented
        
        for error in self.block.iter() 
        {
            //Do stuff
        
        }
        
        output.flush();
        
    }
    
    
    //submits a double to the block for compression
    //Returns tureif the block is fill and should be send to the output stream
    fn submitDouble(&mut self, value:f64 ) -> bool 
    {
     
        if self.blockPos == self.blockSize 
        {
            self.blockPos = 0;
            return true;
        } else {
        
            let error = self.forecaster.error(value);
            self.block[self.blockPos] = error;
            self.blockPos += 1;  
            self.forecaster.train(value, error);
            
           return false; 
        }
    }
    
     //  Returns the number of leading zero bits in the given byte[], 
    //  ignoring the sign bit.
    fn leadingZeroes(bs : &[u8]) -> usize
    {
        let mut first = true;
        let mut ret = 1;
        for b in bs 
        {
            let n = if first {
                        first = false;
                        6//used to ignore sign bit
                    } else {
                        7
                    };
            
            for i in (1..=n).rev()
             {
                let bit = ((b & (1 << i)) >> i);
                if bit == 0 {
                    ret += 1;
                } else {
                    return ret;
                }
            }
        }
        return ret;
    }
    
    fn compressBlock<'a>(&mut self, finish: bool) 
    {
        
        //Doubles are 64 bits to 8 bytes
        const byteWidth: usize = 8; 
        
        //Stores the byte version of each double to be compressed
        let valueBytes: Vec::<[u8; byteWidth]> = Vec::with_capacity(self.blockSize); 
         
        let mut bitTracker = [0; byteWidth];
        
        for i in 0..self.blockSize 
        {
            let valBytes = self.block[i].to_be_bytes();
            
            for b in 0..byteWidth 
            {
                bitTracker[b] = bitTracker[b] | valBytes[0];
            }
        
            
        }
        
        let leadZeros = SprintzCompressor::leadingZeroes(&bitTracker);
        
   
        
    }
    
    
}
