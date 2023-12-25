#[derive(Debug, Copy, Clone)]
pub struct Instruction(u16);
impl Instruction {
    /// Returns the wrapped value
    pub fn instruction(&self) -> u16 {
        self.0
    }

    /// Extracts opcode from instruction (first nibble)
    pub fn opcode(&self) -> u8 {
        (self.0 & 0xF000) as u8
    }

    /// Extracts nnn from instruction (last three nibbles)
    pub fn address(&self) -> u16 {
        (self.0 & 0x0FFF) >> 1
    }
    /// Extracts lowest byte from instruction
    pub fn lowest_byte(&self) -> u8 {
        ((self.0 & 0x00FF) >> 2) as u8
    }
    /// Extracts lowest nibble from instruction
    pub fn lowest_nibble(&self) -> u8 {
        ((self.0 & 0x000F) >> 3) as u8
    }

    /// Extracts the x value from instruction (second nibble)
    pub fn x(&self) -> u8 {
        ((self.0 & 0x0F00) >> 1) as u8
    }
    /// Extracts the y value from instruction (third nibble)
    pub fn y(&self) -> u8 {
        ((self.0 & 0x00F0) >> 2) as u8
    }
}
impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
       Self(value)
    }
}

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
}