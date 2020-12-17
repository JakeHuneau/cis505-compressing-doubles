use std::io::Cursor;
//use std::io::Write;
use std::vec::Vec;
use std::io;

pub struct Forecaster {
    previous: u64,
    
    
}

impl Forecaster {
    
    fn predict(&self, value: u64) -> u64 {
        self.previous
    }
    
    
    fn error(&self, value: u64) -> u64 {
 
        value ^ self.predict(value)
    }
    
    
    //Trains the forecaster for a better prediction
    //Out algorithm just returns the previous value
    fn train(&mut self, value:  u64, error: u64) {
        self.previous = value;
    }
}


/*pub struct SprintzDecoder<'a>
{
    input: &'a Cursor<Vec<u8>>,
    blockSize: usize,
    forecaster: Forecaster,
    readPos: u32,
    leftInBlock: u32,
    zeroesLeft: u64,
    nBits: u32,
    
}


 impl SprintzDecoder<'_> {
    pub fn new<'a>(datainput: &'a Cursor<Vec<u8>>, blockSize: usize)-> SprintzDecoder
    {
         SprintzDecoder 
         {
            input: SprintzInput::new(datainput),
            blockSize,
            forecaster: Forecaster{ previous: 0u64},
            readPos: 0,
            leftInBlock: 0,
            zeroesLeft: 0,
            nBits: 0,
         }
         
    }
    
    
   pub fn readValue(self) -> u64
    {
        if self.leftInBlock > 0 {
            let xor: u64 = self.getLong(1) << 63;
            xor |= self.getLong(self.nBits);
            self.leftInBlock-=1;
            
            let ret = self.forecaster.error(xor);
            self.forecaster.train(xor, ret);
            return ret;
        } else if self.zeroesLeft > 0 {
            self.zeroesLeft-=1;
            return self.forecaster.predict(0);
        } else {
            self.nBits = self.getLong(7) as u32;
            if self. nBits == 0 {
                let numZeroBlocks = self.getLong(16);
                self.zeroesLeft = self.numZeroBlocks * self.blockSize;
            } else {
                self.leftInBlock = self.blockSize;
            }
            
            return self.readValue();
        }
        
    }
    
    fn getBits(self, bits: u32) -> u64 {
        self.pos += bits;
        return self.input.readLong(bits);
    }
}*/


struct SprintzInput<'a> {
    input: &'a Cursor<Vec<u8>>,
    bitsLeft: u8,
    byteBuffer: u8
    
}

impl SprintzInput<'_> {
    
    fn new<'a>(input: &'a Cursor<Vec<u8>>) -> SprintzInput<'a>{
        SprintzInput{
            input,
            bitsLeft: 0,
            bytesBuffer: 0
        }
    }
    
    fn bufferByte(self) -> io::Result<()>{
        if self.bitsLeft == 0 {
            let mut buffer = [u8;1];
            self.input.read_exact(buffer)? ;
            self.byteBuffer = buffer[0];
            self.bitsLeft = 8;
        }
    }
    
    fn readLong(self, bits:u32) -> io::Result<u64> {
        let value = 0u64;
        while bits > 0 {
            if bits > self.bitsLeft || bits == 8 {
                // Take only the bitsLeft "least significant" bits
                let d: u8 = self.byteBuffer & ((1<< self.bitsLeft) - 1);
                value = (value << self.bitsLeft) + (d & 0xFF);
                bits -= self.bitsLeft;
                self.bitsLeft = 0;
            } else {
                // Shift to correct position and take only least significant bits
                let d: u8 =  (self.byteBuffer >> (self.bitsLeft - bits)) & ((1<<bits) - 1);
                value = (value << bits) + (d & 0xFF);
                self.bitsLeft -= bits;
                bits = 0;
            }
            self.bufferByte();
        }
        return value;
    }
}