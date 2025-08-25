/////////////////////////////////////////////
///                MEMORY                 ///
/////////////////////////////////////////////

const MEMORY_SIZE: usize = 4096;

pub struct Memory([u8; MEMORY_SIZE]);

impl Memory {
    pub fn set(&mut self, address: usize, value: u8) {
        if address >= MEMORY_SIZE {
            panic!("out of bounds address access: {:#04x}", address);
        }
        self.0[address] = value;
    }

    pub fn get(&self, address: usize) -> u8 {
        if address >= MEMORY_SIZE {
            panic!("out of bounds address access: {:#04x}", address);
        }
        self.0[address]
    }
}
