#![allow(non_snake_case)]

use super::font::FONT_BOOK;

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

    pub fn cycle(&mut self) -> bool {
        let mut needs_visual_update = false;

        // Fetch
        let opcode = self.next_opcode();
        self.program_counter += 2;
        println!("next opcode: {}", opcode);

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

            (0x1, _, _, _) => self._1NNN_jump(address),

            (0x6, _, _, _) => self._6XNN_set_register(x, constants),

            (0x7, _, _, _) => self._7XNN_add_register(x, constants),

            (0xA, _, _, _) => self._ANNN_set_index_register(address),

            (0xD, _, _, _) => {
                self._DXYN_display(x, y, register);
                needs_visual_update = true;
            },
            _ => {},
        };

        needs_visual_update
    }

    fn next_opcode(&self) -> u16 {
        let pc = self.program_counter;
        (self.memory[pc as usize] as u16) << 8 | (self.memory[pc as usize + 1] as u16)
    }

    fn _00E0_clear_screen(&mut self) {
        self.vram = [[0; VRAM_WIDTH]; VRAM_HEIGHT];
    }

    fn _1NNN_jump(&mut self, register: usize) {
        self.program_counter = register;
    }

    fn _6XNN_set_register(&mut self, register: usize, value: u8) {
        self.variable_registers[register] = value;
    }

    fn _7XNN_add_register(&mut self, register: usize, value: u8) {
        let current_value = self.variable_registers[register];
        self.variable_registers[register] = current_value.overflowing_add(value).0
    }

    fn _ANNN_set_index_register(&mut self, value: usize) {
        self.index_pointer = value;
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
}

#[cfg(test)]
mod tests {
    use crate::hardware::cpu::CPU;

    #[test]
    fn test_decode() {
        let cpu = CPU::new();
    }
}