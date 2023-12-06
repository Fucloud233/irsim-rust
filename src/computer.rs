use std::ops::AddAssign;

const MAX_SIZE: usize = 2^18;

pub struct Computer {
    memory: [u32; MAX_SIZE as usize],  
    pointer_stack: Vec<u32>,  
}

impl Computer {
    pub fn new() -> Self {
        Computer { memory: [0; MAX_SIZE], pointer_stack: vec![0] }
    }

    pub fn load(&self, address: u32) -> u32 {
        self.memory[get_addr(address)]
    }

    pub fn save(&mut self, address: u32, value: u32) {
        self.memory[get_addr(address)] = value;
    }

    pub fn allocate(&mut self, size: u32) -> u32 {
        let pointer = self.pointer_stack.last_mut().unwrap();
        let addr = *pointer * 4;
        pointer.add_assign(size);
        addr
    }

    pub fn push(&mut self) {
        let cur_pointer = self.pointer_stack.last().unwrap();
        self.pointer_stack.push(*cur_pointer);
    }

    pub fn pop(&mut self) {
        self.pointer_stack.pop().unwrap();
    }
}

#[inline]
fn get_addr(offset: u32) -> usize{
    (offset / 4) as usize
} 