#![allow(non_snake_case)]

use bevy::utils::tracing::callsite::register;
use super::font::FONT_BOOK;
use rand::{thread_rng, Rng};

pub const VRAM_WIDTH : usize = 64;
pub const VRAM_HEIGHT: usize = 32;

const MEMORY_SIZE: usize = 4096;
const STACK_SIZE : usize = 16;
const VREG_SIZE  : usize = 16;

pub struct CPU {
    stack : [usize; STACK_SIZE],
    memory: [u8; MEMORY_SIZE],
    vram  : [[u8; VRAM_WIDTH]; VRAM_HEIGHT],

    program_counter   : usize,
    index_pointer     : usize,
    stack_pointer     : usize,
    variable_registers: [u8; VREG_SIZE],

    delay_timer: u8,
    sound_timer: u8,
}
impl CPU {
    pub fn new() -> Self {
        let mut memory = [0; MEMORY_SIZE];
        for i in 0..FONT_BOOK.len() {
            memory[i] = FONT_BOOK[i];
        }

        CPU {
            memory,
            stack             : [0; STACK_SIZE],
            vram              : [[0; VRAM_WIDTH]; VRAM_HEIGHT],
            program_counter   : 0x200,
            index_pointer     : 0,
            stack_pointer     : 0,
            variable_registers: [0; VREG_SIZE],
            delay_timer       : 0,
            sound_timer       : 0,
        }
    }

    pub fn vram_tile(&self, x: u32, y: u32) -> u8 {
        // do this as vram_height - y because bevy_ecs_tilemap starts bottom left and we start top left
        self.vram[VRAM_HEIGHT - 1 - y as usize][x as usize]
    }

    pub fn load_card(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[0x200 + i] = byte;
            } else {
                break;
            }
        }
    }

    pub fn cycle(&mut self, keyboard_input: [u8; 16]) {
        // Fetch
        let opcode = self.next_opcode();
        self.program_counter += 2;

        // Decode
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );

        let address = (opcode & 0x0FFF) as usize;
        let constants = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let register = nibbles.3 as usize;

        // Execute
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self._00E0_clear_screen(),
            (0x0, 0x0, 0xE, 0xE) => self._00EE_pop_jump(),
            (0x1, _, _, _)       => self._1NNN_jump(address),
            (0x2, _, _, _)       => self._2NNN_push_jump(register),
            (0x3, _, _, _)       => self._3XNN_skip_equal(x, constants),
            (0x4, _, _, _)       => self._4XNN_skip_not_equal(x, constants),
            (0x5, _, _, 0x0)     => self._5XY0_skip_registers_equal(x, y),
            (0x6, _, _, _)       => self._6XNN_set_register(x, constants),
            (0x7, _, _, _)       => self._7XNN_add_register(x, constants),
            (0x8, _, _, 0x0)     => self._8XY0_set_x_y(x, y),
            (0x8, _, _, 0x1)     => self._8XY1_or_x_y(x, y),
            (0x8, _, _, 0x2)     => self._8XY2_and_x_y(x, y),
            (0x8, _, _, 0x3)     => self._8XY3_xor_x_y(x, y),
            (0x8, _, _, 0x4)     => self._8XY4_add_x_y(x, y),
            (0x8, _, _, 0x5)     => self._8XY5_subtract_x_y(x, y),
            (0x8, _, _, 0x6)     => self._8XY6_shift_right_x(x),
            (0x8, _, _, 0x7)     => self._8XY7_subtract_y_x(x, y),
            (0x8, _, _, 0xE)     => self._8XYE_shift_left_x(x),
            (0x9, _, _, 0x0)     => self._9XY0_skip_registers_not_equal(x, y),
            (0xA, _, _, _)       => self._ANNN_set_index_register(address),
            (0xB, _, _, _)       => self._BNNN_jump_with_offset(address),
            (0xC, _, _, _)       => self._CXNN_random(x, constants),
            (0xD, _, _, _)       => self._DXYN_display(x, y, register),
            (0xE, _, 0x9, 0xE)   => self._EX9E_skip_key_pressed(x, keyboard_input),
            (0xE, _, 0xA, 0x1)   => self._EXA1_skip_key_not_pressed(x, keyboard_input),
            (0xF, _, 0x0, 0x7)   => self._FX07_set_x_to_timer(x),
            (0xF, _, 0x1, 0x5)   => self._FX15_set_delay_to_x(x),
            (0xF, _, 0x1, 0x8)   => self._FX18_set_sound_to_x(x),
            (0xF, _, 0x1, 0xE)   => self._FX1E_add_x_to_i(x),
            (0xF, _, 0x0, 0xA)   => self._FX0A_blocking_get_key(x, keyboard_input),
            (0xF, _, 0x2, 0x9)   => self._FX29_font_char(x),
            (0xF, _, 0x3, 0x3)   => self._FX33_decimal_conversion(x),
            (0xF, _, 0x5, 0x5)   => self._FX55_store_registers(x),
            (0xF, _, 0x6, 0x5)   => self._FX65_load_registers(x),

            _ => {},
        };
    }

    fn next_opcode(&self) -> u16 {
        let pc = self.program_counter;
        (self.memory[pc as usize] as u16) << 8 | (self.memory[pc as usize + 1] as u16)
    }

    fn _00E0_clear_screen(&mut self) {
        self.vram = [[0; VRAM_WIDTH]; VRAM_HEIGHT];
    }
    fn _00EE_pop_jump(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer];
    }
    fn _1NNN_jump(&mut self, register: usize) {
        self.program_counter = register;
    }
    fn _2NNN_push_jump(&mut self, register: usize) {
        let current_pc = self.program_counter;
        self.stack[self.stack_pointer] = current_pc;
        self.stack_pointer += 1;
        self.program_counter = register;
    }
    fn _3XNN_skip_equal(&mut self, x: usize, value: u8) {
        let register = self.variable_registers[x];
        if register == value {
            self.program_counter += 2;
        }
    }
    fn _4XNN_skip_not_equal(&mut self, x: usize, value: u8) {
        let register = self.variable_registers[x];
        if register != value {
            self.program_counter += 2;
        }
    }
    fn _5XY0_skip_registers_equal(&mut self, x: usize, y: usize) {
        let x_reg = self.variable_registers[x];
        let y_reg = self.variable_registers[y];

        if x_reg == y_reg {
            self.program_counter += 2;
        }
    }
    fn _6XNN_set_register(&mut self, register: usize, value: u8) {
        self.variable_registers[register] = value;
    }
    fn _7XNN_add_register(&mut self, register: usize, value: u8) {
        let current_value = self.variable_registers[register];
        self.variable_registers[register] = current_value.overflowing_add(value).0
    }
    fn _8XY0_set_x_y(&mut self, x: usize, y: usize) {
        self.variable_registers[y] = self.variable_registers[x];
    }
    fn _8XY1_or_x_y(&mut self, x: usize, y: usize) {
        let x_reg = self.variable_registers[x];
        let y_reg = self.variable_registers[y];

        self.variable_registers[x] = x_reg | y_reg;
    }
    fn _8XY2_and_x_y(&mut self, x: usize, y: usize) {
        let x_reg = self.variable_registers[x];
        let y_reg = self.variable_registers[y];

        self.variable_registers[x] = x_reg & y_reg;
    }
    fn _8XY3_xor_x_y(&mut self, x: usize, y: usize) {
        let x_reg = self.variable_registers[x];
        let y_reg = self.variable_registers[y];

        self.variable_registers[x] = x_reg ^ y_reg;
    }
    fn _8XY4_add_x_y(&mut self, x: usize, y: usize) {
        let x_reg = self.variable_registers[x];
        let y_reg = self.variable_registers[y];

        let (result, carry) = x_reg.overflowing_add(y_reg);
        self.variable_registers[x] = result;
        self.variable_registers[0x0F] = if carry { 0x1 } else { 0x0 };
    }
    fn _8XY5_subtract_x_y(&mut self, x: usize, y: usize) {
        let x_reg = self.variable_registers[x];
        let y_reg = self.variable_registers[y];

        let (result, carry) = x_reg.overflowing_sub(y_reg);
        self.variable_registers[x] = result;
        self.variable_registers[0x0F] = if !carry { 0x1 } else { 0x0 };
    }
    fn _8XY6_shift_right_x(&mut self, x: usize) {
        let x_reg = self.variable_registers[x];
        self.variable_registers[0x0F] = x_reg & 1;
        self.variable_registers[x] = x_reg >> 1;
    }
    fn _8XY7_subtract_y_x(&mut self, x: usize, y: usize) {
        let x_reg = self.variable_registers[x];
        let y_reg = self.variable_registers[y];

        let (result, carry) = y_reg.overflowing_sub(x_reg);
        self.variable_registers[x] = result;
        self.variable_registers[0x0F] = if !carry { 0x1 } else { 0x0 };
    }
    fn _8XYE_shift_left_x(&mut self, x: usize) {
        let x_reg = self.variable_registers[x];
        self.variable_registers[0x0F] = (x_reg & 0b10000000) >> 7;
        self.variable_registers[x] = x_reg << 1;
    }
    fn _9XY0_skip_registers_not_equal(&mut self, x: usize, y: usize) {
        let x_reg = self.variable_registers[x];
        let y_reg = self.variable_registers[y];

        if x != y {
            self.program_counter += 2;
        }
    }
    fn _ANNN_set_index_register(&mut self, value: usize) {
        self.index_pointer = value;
    }
    fn _BNNN_jump_with_offset(&mut self, value: usize) {
        let reg = self.variable_registers[0x0];
        self.program_counter = value + reg as usize;
    }
    fn _CXNN_random(&mut self, x: usize, value: u8) {
        let mut rng = rand::thread_rng();
        let num: u8 = rng.gen();
        self.variable_registers[x] = num & value;
    }
    fn _DXYN_display(&mut self, x: usize, y: usize, value: usize) {
        self.variable_registers[0x0F] = 0;

        for byte in 0..value {
            let y = (self.variable_registers[y] as usize + byte) % VRAM_HEIGHT;
            for bit in 0..8 {
                let x = (self.variable_registers[x] as usize + bit) % VRAM_WIDTH;
                let color = (self.memory[self.index_pointer + byte] >> (7 - bit)) & 1;
                self.variable_registers[0x0F] |= color & self.vram[y][x];
                self.vram[y][x] ^= color;
            }
        }
    }
    fn _EX9E_skip_key_pressed(&mut self, x: usize, keys: [u8; 16]) {
        let x_reg = self.variable_registers[x];
        let key = keys[x_reg as usize];
        if key == 1 {
            self.program_counter += 2;
        }
    }
    fn _EXA1_skip_key_not_pressed(&mut self, x: usize, keys: [u8; 16]) {
        let x_reg = self.variable_registers[x];
        let key = keys[x_reg as usize];
        if key != 1 {
            self.program_counter += 2;
        }
    }
    fn _FX07_set_x_to_timer(&mut self, x: usize) {
        self.variable_registers[x] = self.delay_timer;
    }
    fn _FX15_set_delay_to_x(&mut self, x: usize) {
        self.delay_timer = self.variable_registers[x];
    }
    fn _FX18_set_sound_to_x(&mut self, x: usize) {
        self.sound_timer = self.variable_registers[x];
    }
    fn _FX1E_add_x_to_i(&mut self, x: usize) {
        let index = self.index_pointer;
        let x_reg = self.variable_registers[x] as usize;
        let (result, carry) = index.overflowing_add(x_reg);
        self.variable_registers[0x0F] = if carry { 0x1 } else { 0x0 };
        self.index_pointer = result;
    }
    fn _FX0A_blocking_get_key(&mut self, x: usize, keys: [u8; 16]) {
        if keys.iter().any(|k| *k == 1) {
            let k = keys.iter()
                .enumerate()
                .filter(|k| *k.1 == 1)
                .next().unwrap().0;
            self.variable_registers[x] = k as u8;
        } else {
            self.program_counter -= 2;
        }
    }
    fn _FX29_font_char(&mut self, x: usize) {
        self.index_pointer = self.variable_registers[x] as usize;
    }
    fn _FX33_decimal_conversion(&mut self, x: usize) {
        let x_reg = self.variable_registers[x];
        let index = self.index_pointer;

        self.memory[index] = x_reg / 100;
        self.memory[index + 1] = (x_reg % 100) / 10;
        self.memory[index + 2] = x_reg % 10;
    }
    fn _FX55_store_registers(&mut self, x: usize) {
        let registers = &self.variable_registers[0..x];
        for (index, register) in registers.iter().enumerate() {
            self.memory[self.index_pointer + index] = *register;
        }
    }
    fn _FX65_load_registers(&mut self, x: usize) {
        let index_pointer = self.index_pointer;
        let memory = &self.memory[index_pointer..index_pointer + x];
        for (index, register) in memory.iter().enumerate() {
            self.variable_registers[index] = *register;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hardware::cpu::CPU;

    #[test]
    fn test_decode() {
        let cpu = CPU::new();
    }
}