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
    
    zeroBlocks: u16
    compressedBlocks: u16
    
    posInBuf: u8 = 7;
    byteBuf: u8 = 0;
    output: &'a[mut Cursor<Vec<u8>>
    
    
}

 impl SprintzCompressor<'_> {
    
    pub fn new<'a>(input: &'a[f64], blockSize: usize, output: &'a mut Cursor<Vec<u8>> )-> SprintzCompressor{
         SprintzCompressor {
             input,
             block: Vec::with_capacity(blockSize),//u32 into usize 
             blockPos: 0, 
             blockSize: blockSize,
             forecaster: Forecaster{ previous: 0.0}
             output
         }
         
        
    }
    
    fn compress<'a>(&mut self) 

    {
        let len: u32 = self.output.get_ref().len() as u32;
        
        //Write down egnth of data
        let lenBytes: [u8; 4] = le.to_be_bytes();
        self.output.write(&lenBytes);
        
        for val in self.input 
        {
            
           let compress = self.submitDouble(*val);
           
           if compress {
                self.compressBlock(false, output);
            } 
        }
        
        self.compressBlock(true, output);
        int numToAdd = posInBuf + 1;
        if(numToAdd == blockSize) {
            numToAdd = 0;
        }
        
        self.output.write(buf);
        self.output.write(numToAdd);
        
        for error in self.block.iter() 
        {
            //Do stuff
        
        }
        
        self.output.flush();
        
    }
    
    
    //submits a double to the block for compression
    //Returns true if the block is full and should be send to the output stream
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
        //Alternative
        //Use f64.to_bits() - then find leading zero
        //Start from 62 to 0
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
    
    fn compressBlock<'a>(&mut self, finish: bool, output: &mut Cursor<Vec<u8>> ) 
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
        
        let zeros = SprintzCompressor::leadingZeroes(&bitTracker);
        //Max Number of bits used by the diff of values 
        let nBits = 64 - zeros;
        
        
        //When the entire block equals positive 0
        if nBits == 0 && b[0] == 0
        {
            zeroBlocks +=1;
            if finish {
               //Write zero header
                self.bufferBit(0, 7, self.output);
                //Write number of zero bytes
                self.bufferBytes(numZeroBlocks.to_be_bytes(), self.output);
            }
            
        } else {
            if zeroBlocks > 0 {
                //Write zero header
                self.bufferBit(0, 7, self.output);
                //Write number of zero bytes
                self.bufferBytes(numZeroBlocks.to_be_bytes(), self.output);
                zeroBlocks = 0;
            }
            
            //Write signnificatn bits
          
            self.bufferByte(nBits+1, 7, self.output);
            writes = blockPos;
            if (blockPos == 0) {
                writes = blockSize;
            }
            
            //WriteErros
            for(i in 0..writes) {
               // addErrAsNBits(valueBytes[i], nBits);
               
                let signBit = ((err[0] & (1 << 7)) >> 7 == 1);
                self.bufferBit(signBit, self.output);
                
                let numBytes = (nBits + 7) / 8);
               
                let mut first = true;
                for( i in (8 - numBytes)..8) {
                    if (first) {
                        let numHangBits = nBits - ((numBytes - 1) * 8);
                        self.bufferByte(err[i], numHangBits, self.output);
                        first = false;
                    } else {
                        self.bufferByte(err[i], self.output);
                    }
                }
            }
        }
           
    }
    
    
    fn bufferBit(self,bit: bool, output: &mut Cursor<Vec<u8>>) {
        if(posInBuf == -1) {
            //Flush byte when buffer fills
            self.output.write(byteBuff);
            byteBuff = 0;
            posInBuf = 7;
        }
        
        if (bit) {
            buf |= 1 << posInBuf;
        }
        posInBuf--;
        
    }
    
    //  Adds all 8 bits of the given byte to the buffer.
    fn bufferByte(self, data: u8, output: &mut Cursor<Vec<u8>>) {
        self.bufferByte(data, 8, output)
    }
    
  

    
//  Adds the n least significant bits of the given byte to the buffer.
    fn bufferByte (self, data: u8, n:u8, output: &mut Cursor<Vec<u8>>) {
        
        for(i in (0..n).rev() {
            let bit = (data & (1 << i)) == 0;
            self.bufferBit(bit, output);
        }
    }
    
    //  Adds all 8 bits of all of the bytes in the given byte[] to the buffer.
    fn bufferBytes (self, data: &[u8], output: &mut Cursor<Vec<u8>>) {
        for(u8 b : data) {
            self.bufferByte(b, output);
        }
    }
    
    
    
    

    
}
