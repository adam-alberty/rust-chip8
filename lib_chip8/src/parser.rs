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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_nnn() {
        assert_eq!(0x234, nnn(0x1234));
    }
    #[test]
    fn test_parser_n() {
        assert_eq!(0x4, n(0x1234));
    }
    #[test]
    fn test_parser_x() {
        assert_eq!(0x2, x(0x1234));
    }
    #[test]
    fn test_parser_y() {
        assert_eq!(0x3, y(0x1234));
    }
    #[test]
    fn test_parser_kk() {
        assert_eq!(0x34, kk(0x1234));
    }
}
