use crate::hardware::engine::EngineError;

#[derive(Debug, Copy, Clone)]
pub struct Instruction(u16);
impl Instruction {
    /// Returns the wrapped value
    pub fn instruction(&self) -> u16 { self.0 }

    /// Extracts opcode from instruction (first nibble)
    pub fn opcode(&self) -> u8 {
        (self.0 & 0xF000) as u8
    }

    /// Extracts the highest byte from the instruction
    pub fn highest_byte(&self) -> u8 { ((self.0 & 0xFF) >> 4) as u8 }
    /// Extracts lowest byte from instruction
    pub fn lowest_byte(&self) -> u8 { (self.0 & 0x00FF) as u8 }
    /// Extracts nnn from instruction (last three nibbles)
    pub fn address(&self) -> u16 { self.0 & 0x0FFF }

    /// Extracts the x value from instruction (second nibble)
    pub fn x(&self) -> u8 { self.highest_byte() & 0x0F }
    /// Extracts the y value from instruction (third nibble)
    pub fn y(&self) -> u8 { (self.lowest_byte() & 0xF0) >> 4 }
    /// Extracts lowest nibble from instruction
    pub fn lowest_nibble(&self) -> u8 { self.lowest_byte() & 0x0F }

    pub fn instruction_type(&self) -> Result<InstructionType, EngineError> {
        self.try_into()
    }

}
impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
       Self(value)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[allow(non_camel_case_types)]
enum InstructionType {
    /// 0nnn System address call
    SYS,
    /// 00E0 Clear screen
    CLS,
    /// 00EE return from subroutine
    RET,
    /// 1nnn jump addr
    JPA,
    /// 2nnn call addr
    CALL,
    /// 3xkk skip if value equal
    SEV,
    /// 4xkk skip if value not equal
    SNEV,
    /// 5xy0 skip if registers equivalent
    SER,
    /// 6xkk load value into register
    LDV,
    /// 7xkk add value and store in register
    ADDV,
    /// 8xy0 copy value from register x to y
    LDR,
    /// 8xy1 bitwise or x | y
    ORR,
    /// 8xy2 bitwise and x & y
    ANDR,
    /// 8xy3 bitwise xor x ^ y
    XORR,
    /// 8xy4 add and carry register
    ANDR_CARRY,
    /// 8xy5 sub and set borrow
    SUBR_BORROW,
    /// 8xy6 shift right and store
    SHR_STORE,
    /// 8xy7 sub reverse and set borrow
    SUBR_BORROW_REVERSE,
    /// 8xyE shift left and store
    SHL_STORE,
    /// 9xy0 skip next if registers not equal
    SNER,
    /// Annn load addr into 1
    LDA,
    /// Bnnn jump to v0 + nnn
    JPAD,
    /// Cxkk random byte & kk
    RND,
    /// Dxyn draw nibble and set collision
    DRW,
    /// Ex9E skip if key press
    SKPK,
    /// ExA1 skip if key not pressed
    SKPNK,
    /// Fx07 load delay timer into register
    LDD,
    /// Fx0A block execution until key pressed
    LDK,
    /// Fx15 load delay timer from x
    LDDR,
    /// Fx18 load sound timer from x
    LDSR,
    /// Fx1E add I and x register
    ADDI,
    /// Fx29 set I for x register sprite
    LDSPR,
    /// Fx33 store reps in memory
    LDBR,
    /// Fx55 store registers in memory
    LDIR,
    /// Fx65 load registers from memory
    LDMR,
}
impl TryFrom<&Instruction> for InstructionType {
    type Error = EngineError;

    fn try_from(value: &Instruction) -> Result<Self, Self::Error> {
        let match_items = (
            value.opcode(),
            value.x(),
            value.y(),
            value.lowest_nibble(),
        );

        let result = match match_items {
            (0x0, 0x0, 0xE, 0x0) => Self::CLS,
            (0x0, 0x0, 0xE, 0xE) => Self::RET,

            _ => return Err(Self::Error::UnknownInstructionType)
        };

        Ok(result)
    }
}