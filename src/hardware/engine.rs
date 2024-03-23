use crate::hardware::instruction::Instruction;
use super::*;
use super::engine_execute::NextAddress;

const STACK_SIZE: usize = 16;
const MEMORY_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Engine {
    pub(super) program_counter: u16,
    pub(super) stack_pointer: u8,

    pub(super) register_set: [u8; 16],
    pub(super) stack: [u16; STACK_SIZE],
    pub(super) ram: [u8; MEMORY_SIZE],

    pub(super) delay_timer: u8,
    pub(super) sound_timer: u8,
    pub(super) register_i: u16,

    pub(super) internal_display: Vec<u8>,
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
            register_i: 0,

            internal_display: vec![0; 64 * 32],
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

    pub fn tick(&mut self) -> Result<(), EngineError> {
        let instruction = self.fetch();
        let instruction = self.decode(instruction);
        let increment = self.execute(instruction, [false; 16])?;

        match increment {
            NextAddress::NoIncrement => (),
            NextAddress::DefaultIncrement => self.program_counter += 2,
            NextAddress::Increment(i) => self.program_counter += i,
            NextAddress::Set(i) => self.program_counter = i,
        };

        Ok(())
    }

    fn fetch(&self) -> u16 {
        let pc = self.program_counter as usize;

        let mut instruction = 0_u16;
        instruction += (self.ram[pc] as u16) << 8;
        instruction += self.ram[pc + 1] as u16;

        instruction
    }

    fn decode(&self, instruction: u16) -> Instruction {
        let instruction = Instruction::from(instruction);
        debug!("Next instruction: {instruction}");

        instruction
    }
}

#[derive(Debug)]
pub enum EngineError {
    OutOfBounds,
    UnknownInstructionType
}