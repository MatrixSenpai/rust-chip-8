mod constants;
mod display;
mod instructions;
mod instruction_table;

use constants::*;

mod macros {
    macro_rules! mask {
        ($in:ident, $($v:tt),*) => {
            (
                $(
                    mask!($v, $in)
                ),*
            )
        };
        (0, $in: ident) => {
            (($in & 0xF000) >> 12) as usize
        };
        (1, $in:ident) => {
            (($in & 0x0F00) >> 8) as usize
        };
        (2, $in: ident) => {
            (($in & 0x00F0) >> 4) as usize
        };
        (3, $in: ident) => {
            ($in & 0x000F) as usize
        };
        (23, $in: ident) => {
            ($in & 0x00FF) as u8
        };
        (123, $in: ident) => {
            ($in & 0x0FFF) as u16
        };
    }

    pub(crate) use mask;
}

#[derive(Copy, Clone, Debug)]
pub struct Chip8Emulator {
    pub(crate) memory: [u8; MEMORY_SIZE],
    pub display_ram: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],

    pub(crate) v_registers: [u8; REGISTER_COUNT],
    pub(crate) i_register: u16,
    pub(crate) delay_register: u8,
    pub(crate) sound_register: u8,

    pub(crate) program_counter: usize,
    pub(crate) stack_pointer: usize,

    pub(crate) stack: [u16; STACK_SIZE],

    pub key_flags: [bool; KEY_COUNT],
}

impl Chip8Emulator {
    pub fn new(program: &[u8]) -> Self {
        let mut memory = [0; MEMORY_SIZE];

        memory[0..FONT_BOOK.len()].copy_from_slice(FONT_BOOK.as_slice());
        memory[PROGRAM_START..PROGRAM_START+program.len()].copy_from_slice(program);

        Self {
            memory,
            display_ram: [0xFF; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            v_registers: [0; REGISTER_COUNT],
            i_register: 0,
            delay_register: 0,
            sound_register: 0,
            program_counter: PROGRAM_START,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            key_flags: [false; KEY_COUNT],
        }
    }

    pub fn tick(&mut self) {
        let i_first = self.memory[self.program_counter];
        let i_second = self.memory[self.program_counter + 1];

        let instruction = ((i_first as u16) << 8) + (i_second as u16);
        let opcode = ((instruction & 0xF000) >> 12) as usize;

        self.program_counter += 2;
        instruction_table::MAIN_INSTRUCTION_TABLE[opcode].resolve(self, instruction);

        if self.delay_register > 0 { self.delay_register -= 1; }
        if self.sound_register > 0 { self.sound_register -= 1; }
    }

}
