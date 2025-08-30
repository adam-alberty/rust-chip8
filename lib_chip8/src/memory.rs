use crate::config;

pub struct Memory {
    cells: [u8; config::MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Memory {
            cells: [0; config::MEMORY_SIZE],
        };

        for (i, &byte) in config::FONTSET.iter().enumerate() {
            memory.cells[config::FONTSET_START_ADDRESS + i] = byte;
        }

        memory
    }

    pub fn load_rom(&mut self, bytes: &[u8]) -> Result<(), String> {
        let end = config::PROGRAM_START_ADDRESS + bytes.len();
        if end > config::MEMORY_SIZE {
            return Err("Program too large to fit in memory".to_string());
        }
        self.cells[config::PROGRAM_START_ADDRESS..end].copy_from_slice(bytes);
        Ok(())
    }

    pub fn set(&mut self, address: usize, value: u8) {
        if address >= config::MEMORY_SIZE {
            panic!("out of bounds address access: {:#04x}", address);
        }
        self.cells[address] = value;
    }

    pub fn get(&self, address: usize) -> u8 {
        if address >= config::MEMORY_SIZE {
            panic!("out of bounds address access: {:#04x}", address);
        }
        self.cells[address]
    }

    pub fn get_slice(&self, from_address: usize, size: usize) -> &[u8] {
        if from_address + size >= config::MEMORY_SIZE {
            panic!(
                "out of bounds address access: [{:#04x}, {:#04x}]",
                from_address,
                from_address + size - 1
            );
        }
        &self.cells[from_address..from_address + size]
    }

    pub fn get_sprite_address(&self, digit: u8) -> usize {
        if digit > 0xf {
            panic!("out of bound sprite access: {:#02x} (0x0 to 0xf)", digit);
        }
        config::FONTSET_START_ADDRESS + digit as usize * 5
    }
}
