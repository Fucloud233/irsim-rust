const MAX_SIZE: usize = 2^18;

pub struct Computer {
    memory: [u32; MAX_SIZE as usize]    
}

impl Computer {
    pub fn new() -> Self {
        Computer { memory: [0; MAX_SIZE] }
    }

    pub fn load(&self, address: u32) -> u32 {
        self.memory[get_addr(address)]
    }

    pub fn save(&mut self, address: u32, value: u32,) {
        self.memory[get_addr(address)] = value;
    }
}


#[inline]
fn get_addr(offset: u32) -> usize{
    (offset / 4) as usize
} 