use std::ops::AddAssign;
use super::*;

const STACK_SIZE: usize = 16;
const MEMORY_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Engine {
    program_counter: u16,
    stack_pointer: u8,

    register_set: [u8; 16],
    stack: [u16; STACK_SIZE],
    ram: [u8; MEMORY_SIZE],

    delay_timer: u8,
    sound_timer: u8,

    internal_display: Vec<u8>,
}

impl Engine {
    pub fn new() -> Result<Self, EngineError> {
        let mut new_instance = Self {
            program_counter: 0x200,
            stack_pointer: 0,

            register_set: [0; 16],
            stack: [0; STACK_SIZE],
            ram: [0; MEMORY_SIZE],

            delay_timer: 0,
            sound_timer: 0,

            internal_display: Vec::new(),
        };

        new_instance.load_font()?;

        Ok(new_instance)
    }

    pub fn load_game(&mut self, game: Vec<u8>) -> Result<(), EngineError> {
        debug!("Loading game into memory of size {}", &game.len());

        self.load_into_memory(0x200, game)
    }

    fn load_font(&mut self) -> Result<(), EngineError> {
        debug!("Load font requested");
        self.load_into_memory(0, FONT_DATA.to_vec())
    }

    fn load_into_memory(&mut self, starting_location: usize, data: Vec<u8>) -> Result<(), EngineError> {
        debug!("Direct memory load requested of size {} at location 0x{starting_location:x}", &data.len());
        trace!("Dump of memory load requested: {:X?}", data);
        if starting_location + data.len() > MEMORY_SIZE {
            error!(
                "Memory length out of bounds!!! Calculated size {} is longer than {MEMORY_SIZE}",
                starting_location + data.len()
            );
            return Err(EngineError::OutOfBounds)
        }

        for (off, byte) in data.into_iter().enumerate() {
            self.ram[starting_location + off] = byte
        }

        Ok(())
    }

    pub fn tick(&mut self) {
        let instruction = self.fetch();
        let increment = self.decode_and_execute(instruction);

        self.program_counter += instruction;
    }

    fn fetch(&self) -> u16 {
        let pc = self.program_counter as usize;

        let mut instruction = 0_u16;
        instruction += (self.ram[pc] as u16) << 8;
        instruction += self.ram[pc + 1] as u16;

        instruction
    }

    fn decode_and_execute(&mut self, instruction: u16) -> u16 {
        let high_byte = ((instruction & 0xFF00) >> 8) as u8;
        let low_byte = (instruction & 0x00FF) as u8;

        let opcode = (high_byte & 0xF0) >> 4;
        let x = high_byte & 0x0F;
        let y = (low_byte & 0xF0) >> 4;
        let n = low_byte & 0x0F;

        let system_address = instruction & 0x0FFF;

        match (high_byte, low_byte) {

            _ => unreachable!("Unknown instruction!!")
        }

        1
    }

    /// SYS: jump to system address
    fn sys_0nnn(&self, address: u16) {
        warn!("SYS called, NOPing")
    }

    /// CLS: clear screen
    fn cls_0e00(&mut self) {
        self.internal_display = Vec::new()
    }

    /// RET: return from subroutine
    fn ret_00ee(&mut self) {
        self.program_counter = self.stack[self.stack_pointer as usize];
        self.stack_pointer = self.stack_pointer - 1;
    }
}

#[derive(Debug)]
pub enum EngineError {
    OutOfBounds,
}