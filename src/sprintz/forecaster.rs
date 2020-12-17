

pub struct Forecaster {
    previous: u64,
    
    
}

impl Forecaster {
    
    pub fn new() -> Forecaster 
    {
        Forecaster{previous:0}
    }
    
    pub fn predict(&self) -> u64 {
        self.previous
    }
    
    
    pub fn error(&self, value: u64) -> u64 {
 
        value ^ self.predict()
    }
    
    
    //Trains the forecaster for a better prediction
    //Out algorithm just returns the previous value
    pub fn train(&mut self, value:  u64, error: u64) {
        self.previous = value;
    }
}