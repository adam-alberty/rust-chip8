// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
// y - A 4-bit value, the upper 4 bits of the low byte of the instruction
// kk or byte - An 8-bit value, the lowest 8 bits of the instruction

pub fn nnn(instruction: u16) -> u16 {
    instruction & 0x0fff
}

pub fn n(instruction: u16) -> u8 {
    (instruction & 0x000f) as u8
}

pub fn x(instruction: u16) -> u8 {
    ((instruction & 0x0f00) >> 8) as u8
}

pub fn y(instruction: u16) -> u8 {
    ((instruction & 0x00f0) >> 4) as u8
}

pub fn kk(instruction: u16) -> u8 {
    (instruction & 0x00ff) as u8
}
