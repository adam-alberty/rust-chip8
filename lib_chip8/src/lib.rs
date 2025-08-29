use rand::random;

mod cpu;
mod display;
mod keyboard;
mod memory;
mod parser;
mod stack;
pub mod timers;

use cpu::Cpu;
use display::Display;
use keyboard::Keyboard;
use memory::Memory;
use stack::Stack;
use timers::Timers;

pub struct Chip8 {
    cpu: Cpu,
    stack: Stack,
    memory: Memory,
    pub display: Display,
    pub keyboard: Keyboard,
    pub timers: Timers,
}

impl Chip8 {
    //////////////////////////////////////
    ///         PUBLIC INTERFACE       ///
    //////////////////////////////////////

    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            stack: Stack::new(),
            memory: Memory::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
            timers: Timers::new(),
        }
    }

    pub fn load_rom(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.memory.load(bytes)
    }

    pub fn tick(&mut self) {
        self.execute(self.fetch());
    }

    //////////////////////////////////////
    ///    FETCH / DECODE / EXECUTE    ///
    //////////////////////////////////////

    fn fetch(&self) -> u16 {
        let instruction_memory: [u8; 2] = self
            .memory
            .get_slice(self.cpu.pc.get() as usize, 2)
            .try_into()
            .expect("instruction should be 2 bytes");

        u16::from_be_bytes(instruction_memory)
    }

    fn execute(&mut self, instruction: u16) {
        match instruction & 0xf000 {
            0x0000 => match instruction {
                0x00e0 => self.op_clear_display(),
                0x00ee => self.op_return_from_subroutine(),
                _ => panic!("unknown 0x0nnn instruction"),
            },
            0x1000 => self.op_jump(parser::nnn(instruction)),
            0x2000 => self.op_call(parser::nnn(instruction)),
            0x3000 => self.op_skip_equal(parser::x(instruction), parser::kk(instruction)),
            0x4000 => self.op_skip_not_equal(parser::x(instruction), parser::kk(instruction)),
            0x5000 => match instruction & 0x000f {
                0x0 => self.op_skip_equal_registers(parser::x(instruction), parser::y(instruction)),
                _ => panic!("unknown 0x5nnn instruction"),
            },
            0x6000 => self.op_load_value(parser::x(instruction), parser::kk(instruction)),
            0x7000 => self.op_add_value(parser::x(instruction), parser::kk(instruction)),
            0x8000 => match instruction & 0x000f {
                0x0 => self.op_load_register(parser::x(instruction), parser::y(instruction)),
                0x1 => self.op_bitwise_or(parser::x(instruction), parser::y(instruction)),
                0x2 => self.op_bitwise_and(parser::x(instruction), parser::y(instruction)),
                0x3 => self.op_bitwise_xor(parser::x(instruction), parser::y(instruction)),
                0x4 => self.op_add_register(parser::x(instruction), parser::y(instruction)),
                0x5 => self.op_subtract_register(parser::x(instruction), parser::y(instruction)),
                0x6 => self.op_shift_right(parser::x(instruction), parser::y(instruction)),
                0x7 => self.op_subtract_negative(parser::x(instruction), parser::y(instruction)),
                0xe => self.op_shift_left(parser::x(instruction), parser::y(instruction)),
                _ => panic!("unknown 0x8nnn instruction"),
            },
            0x9000 => match instruction & 0x000f {
                0x0 => {
                    self.op_skip_not_equal_registers(parser::x(instruction), parser::y(instruction))
                }
                _ => panic!("unknown 0x9nnn instruction"),
            },
            0xa000 => self.op_set_i_register(parser::nnn(instruction)),
            0xb000 => self.op_jump_to_v0_plus_addr(parser::nnn(instruction)),
            0xc000 => self.op_random(parser::x(instruction), parser::kk(instruction)),
            0xd000 => self.op_display_sprite(
                parser::x(instruction),
                parser::y(instruction),
                parser::n(instruction),
            ),
            0xe000 => match instruction & 0x00ff {
                0x9e => self.op_skip_key_pressed(parser::x(instruction)),
                0xa1 => self.op_skip_key_not_pressed(parser::x(instruction)),
                _ => panic!("unknown 0xennn instruction"),
            },
            0xf000 => match instruction & 0x00ff {
                0x07 => self.op_load_delay_timer(parser::x(instruction)),
                0x0a => self.op_wait_for_key_press(parser::x(instruction)),
                0x15 => self.op_set_delay_timer(parser::x(instruction)),
                0x18 => self.op_set_sound_timer(parser::x(instruction)),
                0x1e => self.op_add_i(parser::x(instruction)),
                0x29 => self.op_set_sprite_location(parser::x(instruction)),
                0x33 => self.op_load_bcd(parser::x(instruction)),
                0x55 => self.op_store_registers(parser::x(instruction)),
                0x65 => self.op_load_registers(parser::x(instruction)),
                _ => panic!("unknown 0xfnnn instruction"),
            },
            _ => panic!("unknown instruction {:#04x}", instruction),
        }
    }

    //////////////////////////////////////
    ///           OPERATIONS           ///
    //////////////////////////////////////

    // 00E0 - CLS
    fn op_clear_display(&mut self) {
        self.display.clear();
        self.cpu.pc.advance();
    }

    // 00EE - RET
    fn op_return_from_subroutine(&mut self) {
        self.cpu.pc.set(self.stack.pop());
    }

    // 1nnn - JP addr
    fn op_jump(&mut self, address: u16) {
        self.cpu.pc.set(address);
    }

    // 2nnn - CALL addr
    fn op_call(&mut self, address: u16) {
        self.cpu.pc.advance();
        self.stack.push(self.cpu.pc.get());
        self.cpu.pc.set(address);
    }

    // 3xkk - SE Vx, byte
    fn op_skip_equal(&mut self, x: u8, byte: u8) {
        if self.cpu.v.get(x) == byte {
            self.cpu.pc.advance();
        }
        self.cpu.pc.advance();
    }

    // 4xkk - SNE Vx, byte
    fn op_skip_not_equal(&mut self, x: u8, byte: u8) {
        if self.cpu.v.get(x) != byte {
            self.cpu.pc.advance();
        }
        self.cpu.pc.advance();
    }

    // 5xy0 - SE Vx, Vy
    fn op_skip_equal_registers(&mut self, x: u8, y: u8) {
        if self.cpu.v.get(x) == self.cpu.v.get(y) {
            self.cpu.pc.advance();
        }
        self.cpu.pc.advance();
    }

    // 6xkk - LD Vx, byte
    fn op_load_value(&mut self, x: u8, value: u8) {
        self.cpu.v.set(x, value);
        self.cpu.pc.advance();
    }

    // 7xkk - ADD Vx, byte
    fn op_add_value(&mut self, x: u8, value: u8) {
        self.cpu.v.set(x, self.cpu.v.get(x).wrapping_add(value));
        self.cpu.pc.advance();
    }

    // 8xy0 - LD Vx, Vy
    fn op_load_register(&mut self, x: u8, y: u8) {
        self.cpu.v.set(x, self.cpu.v.get(y));
        self.cpu.pc.advance();
    }

    // 8xy1 - OR Vx, Vy
    fn op_bitwise_or(&mut self, x: u8, y: u8) {
        self.cpu.v.set(x, self.cpu.v.get(x) | self.cpu.v.get(y));
        self.cpu.v.set(0xf, 0);
        self.cpu.pc.advance();
    }

    // 8xy2 - AND Vx, Vy
    fn op_bitwise_and(&mut self, x: u8, y: u8) {
        self.cpu.v.set(x, self.cpu.v.get(x) & self.cpu.v.get(y));
        self.cpu.v.set(0xf, 0);
        self.cpu.pc.advance();
    }

    // 8xy3 - XOR Vx, Vy
    fn op_bitwise_xor(&mut self, x: u8, y: u8) {
        self.cpu.v.set(x, self.cpu.v.get(x) ^ self.cpu.v.get(y));
        self.cpu.v.set(0xf, 0);
        self.cpu.pc.advance();
    }

    // 8xy4 - ADD Vx, Vy
    fn op_add_register(&mut self, x: u8, y: u8) {
        let (value, carry) = self.cpu.v.get(x).overflowing_add(self.cpu.v.get(y));
        self.cpu.v.set(x, value);
        self.cpu.v.set(0xf, carry as u8);
        self.cpu.pc.advance();
    }

    // 8xy5 - SUB Vx, Vy
    fn op_subtract_register(&mut self, x: u8, y: u8) {
        let vf_value = self.cpu.v.get(x) >= self.cpu.v.get(y);
        self.cpu
            .v
            .set(x, self.cpu.v.get(x).wrapping_sub(self.cpu.v.get(y)));
        self.cpu.v.set(0xf, vf_value as u8);
        self.cpu.pc.advance();
    }

    // 8xy6 - SHR Vx {, Vy}
    fn op_shift_right(&mut self, x: u8, y: u8) {
        let value = self.cpu.v.get(y);
        self.cpu.v.set(x, value >> 1);
        self.cpu.v.set(0xf, value & 1);
        self.cpu.pc.advance();
    }

    // 8xy7 - SUBN Vx, Vy
    fn op_subtract_negative(&mut self, x: u8, y: u8) {
        let vf_value = self.cpu.v.get(y) >= self.cpu.v.get(x);
        self.cpu
            .v
            .set(x, self.cpu.v.get(y).wrapping_sub(self.cpu.v.get(x)));
        self.cpu.v.set(0xf, vf_value as u8);
        self.cpu.pc.advance();
    }

    // 8xyE - SHL Vx {, Vy}
    fn op_shift_left(&mut self, x: u8, y: u8) {
        let value = self.cpu.v.get(y);
        self.cpu.v.set(x, value << 1);
        self.cpu.v.set(0xf, (value >> 7) & 1);
        self.cpu.pc.advance();
    }

    // 9xy0 - SNE Vx, Vy
    fn op_skip_not_equal_registers(&mut self, x: u8, y: u8) {
        if self.cpu.v.get(x) != self.cpu.v.get(y) {
            self.cpu.pc.advance();
        }
        self.cpu.pc.advance();
    }

    // Annn - LD I, addr
    fn op_set_i_register(&mut self, addr: u16) {
        self.cpu.i.set(addr);
        self.cpu.pc.advance();
    }

    // Bnnn - JP V0, addr
    fn op_jump_to_v0_plus_addr(&mut self, addr: u16) {
        self.cpu.pc.set(self.cpu.v.get(0) as u16 + addr);
    }

    // Cxkk - RND Vx, byte
    fn op_random(&mut self, x: u8, byte: u8) {
        self.cpu.v.set(x, random::<u8>() & byte);
        self.cpu.pc.advance();
    }

    // Dxyn - DRW Vx, Vy, nibble
    fn op_display_sprite(&mut self, x: u8, y: u8, size: u8) {
        let x = self.cpu.v.get(x) as usize;
        let y = self.cpu.v.get(y) as usize;
        let sprite_bytes = self
            .memory
            .get_slice(self.cpu.i.get() as usize, size as usize);
        self.cpu
            .v
            .set(0xf, self.display.display_sprite(x, y, sprite_bytes) as u8);
        self.cpu.pc.advance();
    }

    // Ex9E - SKP Vx
    fn op_skip_key_pressed(&mut self, x: u8) {
        if self.keyboard.is_pressed(self.cpu.v.get(x)) {
            self.cpu.pc.advance();
        }
        self.cpu.pc.advance();
    }

    // ExA1 - SKNP Vx
    fn op_skip_key_not_pressed(&mut self, x: u8) {
        if !self.keyboard.is_pressed(self.cpu.v.get(x)) {
            self.cpu.pc.advance();
        }
        self.cpu.pc.advance();
    }

    // Fx07 - LD Vx, DT
    fn op_load_delay_timer(&mut self, x: u8) {
        self.cpu.v.set(x, self.timers.get(timers::Timer::Delay));
        self.cpu.pc.advance();
    }

    // Fx0A - LD Vx, K
    fn op_wait_for_key_press(&mut self, x: u8) {
        if let Some(key) = self.keyboard.get_pressed_key() {
            self.cpu.v.set(x, key);
            self.cpu.pc.advance();
        }
    }

    // Fx15 - LD DT, Vx
    fn op_set_delay_timer(&mut self, x: u8) {
        self.timers.set(timers::Timer::Delay, self.cpu.v.get(x));
        self.cpu.pc.advance();
    }

    // Fx18 - LD ST, Vx
    fn op_set_sound_timer(&mut self, x: u8) {
        self.timers.set(timers::Timer::Sound, self.cpu.v.get(x));
        self.cpu.pc.advance();
    }

    // Fx1E - ADD I, Vx
    fn op_add_i(&mut self, x: u8) {
        self.cpu
            .i
            .set(self.cpu.i.get().wrapping_add(self.cpu.v.get(x) as u16));
        self.cpu.pc.advance();
    }

    // Fx29 - LD F, Vx
    fn op_set_sprite_location(&mut self, x: u8) {
        self.cpu
            .i
            .set(self.memory.get_sprite_address(self.cpu.v.get(x)) as u16);
        self.cpu.pc.advance();
    }

    // Fx33 - LD B, Vx
    fn op_load_bcd(&mut self, x: u8) {
        self.memory
            .set(self.cpu.i.get() as usize, (self.cpu.v.get(x) / 100) % 10);
        self.memory
            .set(self.cpu.i.get() as usize + 1, (self.cpu.v.get(x) / 10) % 10);
        self.memory
            .set(self.cpu.i.get() as usize + 2, self.cpu.v.get(x) % 10);
        self.cpu.pc.advance();
    }

    // Fx55 - LD [I], Vx
    fn op_store_registers(&mut self, x: u8) {
        let i = self.cpu.i.get();
        for idx in 0..=x {
            self.memory
                .set(i as usize + idx as usize, self.cpu.v.get(idx));
        }
        // quirk
        self.cpu.i.set(i + (x as u16) + 1);

        self.cpu.pc.advance();
    }

    // Fx65 - LD Vx, [I]
    fn op_load_registers(&mut self, x: u8) {
        let i = self.cpu.i.get();
        for idx in 0..=x {
            self.cpu
                .v
                .set(idx, self.memory.get(i as usize + idx as usize));
        }
        // quirk
        self.cpu.i.set(i + (x as u16) + 1);

        self.cpu.pc.advance();
    }
}
