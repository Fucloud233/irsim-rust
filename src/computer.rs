const MAX_SIZE: usize = 2e18 as usize;

pub struct Computer {
    memory: [u8; MAX_SIZE]    
}

impl Computer {
    pub fn new() -> Self {
        Computer { memory: [0; MAX_SIZE] }
    }
}