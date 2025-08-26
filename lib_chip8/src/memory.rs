/////////////////////////////////////////////
///                MEMORY                 ///
/////////////////////////////////////////////

const FONTSET: [u8; 80] = [
    // 0
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 1
    0x20, 0x60, 0x20, 0x20, 0x70, // 2
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 3
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 4
    0x90, 0x90, 0xF0, 0x10, 0x10, // 5
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 6
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 7
    0xF0, 0x10, 0x20, 0x40, 0x40, // 8
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // A
    0xF0, 0x90, 0xF0, 0x90, 0x90, // B
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // C
    0xF0, 0x80, 0x80, 0x80, 0xF0, // D
    0xE0, 0x90, 0x90, 0x90, 0xE0, // E
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // F
    0xF0, 0x80, 0xF0, 0x80, 0x80,
];
const MEMORY_SIZE: usize = 4096;
const FONTSET_START_ADDRESS: usize = 0x0;
const PROGRAM_START_ADDRESS: usize = 0x200;

pub struct Memory([u8; MEMORY_SIZE]);

impl Memory {
    pub fn new() -> Self {
        let mut memory = Memory([0; MEMORY_SIZE]);

        for (i, &byte) in FONTSET.iter().enumerate() {
            memory.0[FONTSET_START_ADDRESS + i] = byte;
        }

        memory
    }

    pub fn load(&mut self, bytes: &[u8]) -> Result<(), String> {
        let end = PROGRAM_START_ADDRESS + bytes.len();
        if end > MEMORY_SIZE {
            return Err("Program too large to fit in memory".to_string());
        }
        self.0[PROGRAM_START_ADDRESS..end].copy_from_slice(bytes);
        Ok(())
    }

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

    pub fn get_slice(&self, from_address: usize, size: usize) -> &[u8] {
        if from_address + size >= MEMORY_SIZE {
            panic!(
                "out of bounds address access: [{:#04x}, {:#04x}]",
                from_address,
                from_address + size - 1
            );
        }
        &self.0[from_address..from_address + size]
    }

    pub fn get_sprite_address(&self, digit: u8) -> usize {
        if digit > 0xf {
            panic!("out of bound sprite access: {:#02x} (0x0 to 0xf)", digit);
        }
        FONTSET_START_ADDRESS + digit as usize * 5
    }

    pub fn get_sprite(&self, digit: u8) -> &[u8] {
        let start_address = self.get_sprite_address(digit);
        &self.0[start_address..start_address + 5]
    }
}
