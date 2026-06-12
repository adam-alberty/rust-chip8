use crate::{config, memory::MemoryError::ProgramTooLarge};
use thiserror::Error;

pub struct Memory {
    cells: [u8; config::MEMORY_SIZE],
}

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("program too large")]
    ProgramTooLarge,

    #[error("out of bounds access: {0}")]
    OutOfBounds(usize),

    #[error("out of bound sprite access: {0} (0x0 to 0xf)")]
    SpriteOutOfBounds(u8),
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

    pub fn load_rom(&mut self, bytes: &[u8]) -> Result<(), MemoryError> {
        let end = config::PROGRAM_START_ADDRESS + bytes.len();
        if end > config::MEMORY_SIZE {
            return Err(ProgramTooLarge);
        }
        self.cells[config::PROGRAM_START_ADDRESS..end].copy_from_slice(bytes);
        Ok(())
    }

    pub fn set(&mut self, address: usize, value: u8) -> Result<(), MemoryError> {
        if address >= config::MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds(address));
        }
        self.cells[address] = value;

        Ok(())
    }

    pub fn get(&self, address: usize) -> Result<u8, MemoryError> {
        if address >= config::MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds(address));
        }
        Ok(self.cells[address])
    }

    pub fn get_slice(&self, from_address: usize, size: usize) -> Result<&[u8], MemoryError> {
        if from_address + size >= config::MEMORY_SIZE {
            return Err(MemoryError::OutOfBounds(from_address + size - 1));
        }
        Ok(&self.cells[from_address..from_address + size])
    }

    pub fn get_sprite_address(&self, digit: u8) -> Result<usize, MemoryError> {
        if digit > 0xf {
            return Err(MemoryError::SpriteOutOfBounds(digit));
        }
        Ok(config::FONTSET_START_ADDRESS + digit as usize * 5)
    }
}
