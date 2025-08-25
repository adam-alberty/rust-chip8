use rand::random;

mod cpu;
mod display;
mod keyboard;
mod memory;
mod stack;
mod timers;

use cpu::Cpu;
use display::Display;
use keyboard::Keyboard;
use memory::Memory;
use stack::Stack;
use timers::Timers;

struct Chip8 {
    cpu: Cpu,
    stack: Stack,
    memory: Memory,
    display: Display,
    keyboard: Keyboard,
    timers: Timers,
}

impl Chip8 {
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

    // LOAD - 0x6xkk
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
        self.cpu.pc.advance();
    }

    // 8xy2 - AND Vx, Vy
    fn op_bitwise_and(&mut self, x: u8, y: u8) {
        self.cpu.v.set(x, self.cpu.v.get(x) & self.cpu.v.get(y));
        self.cpu.pc.advance();
    }

    // 8xy3 - XOR Vx, Vy
    fn op_bitwise_xor(&mut self, x: u8, y: u8) {
        self.cpu.v.set(x, self.cpu.v.get(x) ^ self.cpu.v.get(y));
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
        self.cpu
            .v
            .set(0xf, (self.cpu.v.get(x) > self.cpu.v.get(y)) as u8);
        self.cpu
            .v
            .set(x, self.cpu.v.get(x).wrapping_sub(self.cpu.v.get(y)));

        self.cpu.pc.advance();
    }

    // 8xy6 - SHR Vx {, Vy}
    fn op_shift_right(&mut self, x: u8) {
        self.cpu.v.set(0xf, self.cpu.v.get(x) & 1);
        self.cpu.v.set(x, self.cpu.v.get(x) >> 1);
        self.cpu.pc.advance();
    }

    // 8xy7 - SUBN Vx, Vy
    fn op_subtract_negative(&mut self, x: usize, y: usize) {
        self.v[0xf] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        self.next_instruction();
    }

    // SHL - 0x8xyE
    fn op_shift_left(&mut self, x: usize) {
        self.v[0xf] = self.v[x] & 1;
        self.v[x] <<= 1;
        self.next_instruction();
    }

    // SNE - 0x9xy0
    fn op_skip_not_equal_registers(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.next_instruction();
        }
        self.next_instruction();
    }

    // LOAD I - 0xAnnn
    fn op_set_i_register(&mut self, addr: u16) {
        self.i_register = addr;
        self.next_instruction();
    }

    // JMP - 0xBnnn
    fn op_jump_to_v0_plus_addr(&mut self, addr: u16) {
        let final_address = addr + self.v[0] as u16;
        self.pc = final_address;
    }

    // RND Vx, byte - 0xCxkk
    fn op_random(&mut self, x: usize, byte: u8) {
        let rnd: u8 = random();
        self.v[x] = rnd & byte;
    }

    // Dxyn - DRW Vx, Vy, nibble
    // TODO implement later
    fn op_display_sprite(&mut self, x: usize, y: usize, size: u8) {
        let coordinates = (self.v[x], self.v[y]);
    }

    // Ex9E - SKP Vx
    fn op_skip_pressed(&mut self, x: usize) {
        let key = self.v[x];

        if key > 0xf {
            self.next_instruction();
            return;
        }

        if self.keyboard.0[key as usize] {
            self.next_instruction();
        }
        self.next_instruction();
    }

    // ExA1 - SKNP Vx
    fn op_skip_not_pressed(&mut self, x: usize) {
        let key = self.v[x];

        if key > 0xf {
            self.next_instruction();
            return;
        }

        if !self.keyboard.0[key as usize] {
            self.next_instruction();
        }
        self.next_instruction();
    }

    // Fx07 - LD Vx, DT
    fn op_load_delay_timer(&mut self, x: usize) {
        self.v[x] = self.timers.delay;
        self.next_instruction();
    }

    // Fx0A - LD Vx, K
    fn op_wait_for_key_press(&mut self, x: usize) {
        // TODO
    }

    // Fx07 - LD Vx, DT
    fn op_set_delay_timer(&mut self, x: usize) {
        self.timers.delay = self.v[x];
        self.next_instruction();
    }

    // Fx07 - LD Vx, DT
    fn op_set_sound_timer(&mut self, x: usize) {
        self.timers.sound = self.v[x];
        self.next_instruction();
    }

    // Fx1E - ADD I, Vx
    fn op_add_i(&mut self, x: usize) {
        self.i_register.wrapping_add(self.v[x] as u16);
        self.next_instruction();
    }

    // Fx29 - LD F, Vx
    fn op_set_sprite_location(&mut self, x: usize) {
        todo!();
    }

    // Fx33 - LD B, Vx
    fn op_load_bcd(&mut self, x: usize) {
        self.memory[self.i_register as usize] = (self.v[x] / 100) % 10;
        self.memory[self.i_register as usize + 1] = (self.v[x] / 10) % 10;
        self.memory[self.i_register as usize + 2] = self.v[x] % 10;
    }

    // Fx55 - LD [I], Vx
    fn op_store_registers(&mut self, x: usize) {
        for i in 0..=x {
            self.memory.set(self.i_register as usize + i, self.v[i]);
        }
        self.next_instruction();
    }

    // Fx65 - LD Vx, [I]
    fn op_load_registers(&mut self, x: usize) {
        for i in 0..=x {
            self.v[i] = self.memory.get(self.i_register as usize + i);
        }
    }
}
