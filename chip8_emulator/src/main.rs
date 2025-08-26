use lib_chip8::Chip8;

fn main() {
    let mut chip = Chip8::new();

    let rom_path = std::env::args().nth(1).expect("provide ROM file path");
    let rom_bytes: Vec<u8> = std::fs::read(rom_path).expect("Failed to read ROM");

    chip.load_rom(&rom_bytes).unwrap_or_else(|e| {
        panic!("Failed to load rom: {}", e);
    });
}
