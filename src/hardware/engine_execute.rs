use std::ops::AddAssign;
use rand::random;
use super::{
    engine::*,
    instruction::*,
};

pub type KeyboardSet = [bool; 16];

pub(super) enum NextAddress {
    NoIncrement,
    DefaultIncrement,
    Increment(u16),
    Set(u16),
}

impl Engine {
    pub(super) fn execute(&mut self, instruction: Instruction, keys: KeyboardSet) -> Result<NextAddress, EngineError> {
        let instruction_type = instruction.instruction_type()?;

        let next_address = match instruction_type {
            InstructionType::SYS => self.sys_0nnn(),
            InstructionType::CLS => self.cls_00e0(),
            InstructionType::RET => self.ret_00ee(),
            InstructionType::JPA => self.jp_1nnn(instruction.address()),
            InstructionType::CALL => self.call_2nnn(instruction.address()),
            InstructionType::SEV => self.se_3xkk(instruction.x(), instruction.lowest_byte()),
            InstructionType::SNEV => self.sne_4xkk(instruction.x(), instruction.lowest_byte()),
            InstructionType::SER => self.se_5xy0(instruction.x(), instruction.y()),
            InstructionType::LDV => self.ld_6xkk(instruction.x(), instruction.lowest_byte()),
            InstructionType::ADDV => self.add_7xkk(instruction.x(), instruction.lowest_byte()),
            InstructionType::LDR => self.ld_8xy0(instruction.x(), instruction.y()),
            InstructionType::ORR => self.or_8xy1(instruction.x(), instruction.y()),
            InstructionType::ANDR => self.and_8xy2(instruction.x(), instruction.y()),
            InstructionType::XORR => self.xor_8xy3(instruction.x(), instruction.y()),
            InstructionType::ANDR_CARRY => self.add_8xy4(instruction.x(), instruction.y()),
            InstructionType::SUBR_BORROW => self.sub_8xy5(instruction.x(), instruction.y()),
            InstructionType::SHR_STORE => self.shr_8xy6(instruction.x(), instruction.y()),
            InstructionType::SUBR_BORROW_REVERSE => self.sub_8xy7(instruction.x(), instruction.y()),
            InstructionType::SHL_STORE => self.shl_8xye(instruction.x(), instruction.y()),
            InstructionType::SNER => self.sne_9xy0(instruction.x(), instruction.y()),
            InstructionType::LDA => self.ldi_annn(instruction.address()),
            InstructionType::JPAD => self.jpa_bnnn(instruction.address()),
            InstructionType::RND => self.rnd_cxkk(instruction.x(), instruction.lowest_byte()),
            InstructionType::DRW => self.drw_dxyn(instruction.x(), instruction.y(), instruction.lowest_nibble()),
            InstructionType::SKPK => self.skk_ex9e(instruction.x(), keys),
            InstructionType::SKPNK => self.sknk_exa1(instruction.x(), keys),
            InstructionType::LDD => self.ldd_fx07(instruction.x()),
            InstructionType::LDK => self.ldk_fx0a(instruction.x(), keys),
            InstructionType::LDDR => self.ldd_fx15(instruction.x()),
            InstructionType::LDSR => self.lds_fx18(instruction.x()),
            InstructionType::ADDI => self.adi_fx1e(instruction.x()),
            InstructionType::LDSPR => self.ldi_fx29(instruction.x()),
            InstructionType::LDBR => self.str_fx33(instruction.x()),
            InstructionType::LDIR => self.str_fx55(instruction.x()),
            InstructionType::LDMR => self.ldr_fx65(instruction.x()),
        };

        Ok(next_address)
    }

    /// 0nnn Jump to machine code routine. This is ignored
    fn sys_0nnn(&self) -> NextAddress {
        warn!("SYS was called, this is ignored");

        NextAddress::DefaultIncrement
    }

    /// 00E0 Clear the display
    fn cls_00e0(&mut self) -> NextAddress {
        self.internal_display = Vec::new();

        NextAddress::DefaultIncrement
    }

    /// 00EE Return from subroutine
    fn ret_00ee(&mut self) -> NextAddress {
        let address = self.stack[self.stack_pointer as usize];
        self.stack_pointer -= 1;

        NextAddress::Set(address)
    }

    /// 1nnn Jump to address
    fn jp_1nnn(&mut self, address: u16) -> NextAddress { NextAddress::Set(address) }

    /// 2nnn Call address
    fn call_2nnn(&mut self, address: u16) -> NextAddress {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;

        NextAddress::Set(address)
    }

    /// 3xkk Skip next instruction if
    fn se_3xkk(&self, register: u8, byte: u8) -> NextAddress {
        let register_value = self.register_set[register as usize];

        if register_value == byte {
            NextAddress::Increment(4)
        } else {
            NextAddress::DefaultIncrement
        }
    }

    /// 4xkk Skip next instruction if not
    fn sne_4xkk(&self, register: u8, byte: u8) -> NextAddress {
        let register_value = self.register_set[register as usize];

        if register_value == byte {
            NextAddress::DefaultIncrement
        } else {
            NextAddress::Increment(4)
        }
    }

    /// 5xy0 Skip if registers equivalent
    fn se_5xy0(&self, register_x: u8, register_y: u8) -> NextAddress {
        let register_x = self.register_set[register_x as usize];
        let register_y = self.register_set[register_y as usize];

        if register_x == register_y {
            NextAddress::Increment(4)
        } else {
            NextAddress::DefaultIncrement
        }
    }

    /// 6xkk Load register
    fn ld_6xkk(&mut self, register: u8, value: u8) -> NextAddress {
        self.register_set[register as usize] = value;

        NextAddress::DefaultIncrement
    }

    /// 7xkk Add to register
    fn add_7xkk(&mut self, register: u8, value: u8) -> NextAddress {
        let mut register_value = self.register_set[register as usize];
        register_value.add_assign(value);
        self.register_set[register as usize] = register_value;

        NextAddress::DefaultIncrement
    }

    /// 8xy0 Load y into x
    fn ld_8xy0(&mut self, register_x: u8, register_y: u8) -> NextAddress {
        self.register_set[register_x as usize] = self.register_set[register_y as usize];

        NextAddress::DefaultIncrement
    }

    /// 8xy1 Set x to x OR y
    fn or_8xy1(&mut self, register_x: u8, register_y: u8) -> NextAddress {
        let vx = self.register_set[register_x as usize];
        let vy = self.register_set[register_y as usize];

        self.register_set[register_x as usize] = vx | vy;

        NextAddress::DefaultIncrement
    }

    /// 8xy2 Set x to x AND y
    fn and_8xy2(&mut self, register_x: u8, register_y: u8) -> NextAddress {
        let vx = self.register_set[register_x as usize];
        let vy = self.register_set[register_y as usize];

        self.register_set[register_x as usize] = vx & vy;

        NextAddress::DefaultIncrement
    }

    /// 8xy3 Set x to x XOR y
    fn xor_8xy3(&mut self, register_x: u8, register_y: u8) -> NextAddress {
        let vx = self.register_set[register_x as usize];
        let vy = self.register_set[register_y as usize];

        self.register_set[register_x as usize] = vx ^ vy;

        NextAddress::DefaultIncrement
    }

    /// 8xy4 Add x and y, set register 0xF if carry occurred
    fn add_8xy4(&mut self, register_x: u8, register_y: u8) -> NextAddress {
        let vx = self.register_set[register_x as usize];
        let vy = self.register_set[register_y as usize];

        let (_, did_carry) = vx.overflowing_add(vy);
        let result = ((vx as u16) + (vy as u16) & 0x00FF) as u8;

        self.register_set[register_x as usize] = result;
        self.register_set[0xF] = if did_carry { 1 } else { 0 };

        NextAddress::DefaultIncrement
    }

    /// 8xy5 Subtract y from x, set register 0xF if borrow did not occur
    fn sub_8xy5(&mut self, register_x: u8, register_y: u8) -> NextAddress {
        let vx = self.register_set[register_x as usize];
        let vy = self.register_set[register_y as usize];

        let (result, did_borrow) = vy.overflowing_sub(vx);
        self.register_set[register_x as usize] = result;
        self.register_set[0xF] = if did_borrow { 0 } else { 1 };

        NextAddress::DefaultIncrement
    }

    /// 8xy6 Multiply x by 2, set register 0xF if LSB of value is 1
    fn shr_8xy6(&mut self, register_x: u8, _register_y: u8) -> NextAddress {
        let vx = self.register_set[register_x as usize];

        self.register_set[register_x as usize] = vx * 2;
        self.register_set[0xF] = vx & 1;

        NextAddress::DefaultIncrement
    }

    /// 8xy7 Subtract x from y, set register 0xF if borrow did not occur
    fn sub_8xy7(&mut self, register_x: u8, register_y: u8) -> NextAddress {
        let vx = self.register_set[register_x as usize];
        let vy = self.register_set[register_y as usize];

        let (result, did_borrow) = vx.overflowing_sub(vy);
        self.register_set[register_x as usize] = result;
        self.register_set[0xF] = if did_borrow { 0 } else { 1 };

        NextAddress::DefaultIncrement
    }

    /// 8xyE Divide x by 2, set register 0xF if MSB of value is 1
    fn shl_8xye(&mut self, register_x: u8, _register_y: u8) -> NextAddress {
        let vx = self.register_set[register_x as usize];

        self.register_set[register_x as usize] = vx / 2;
        self.register_set[0xF] = vx & 0xF0;

        NextAddress::DefaultIncrement
    }

    /// 9xy0 Skip next instruction if x and y are not equivalent
    fn sne_9xy0(&mut self, register_x: u8, register_y: u8) -> NextAddress {
        let vx = self.register_set[register_x as usize];
        let vy = self.register_set[register_y as usize];

        if vx == vy {
            NextAddress::DefaultIncrement
        } else {
            NextAddress::Increment(4)
        }
    }

    /// Annn Set I register to value nnn
    fn ldi_annn(&mut self, address: u16) -> NextAddress {
        self.register_i = address;

        NextAddress::DefaultIncrement
    }

    /// Bnnn Jump to address nnn + value of register 0
    fn jpa_bnnn(&mut self, address: u16) -> NextAddress {
        let register_value = self.register_set[0x0] as u16;

        NextAddress::Set(register_value + address)
    }

    /// Cxkk Generate random number and set x to value & number
    fn rnd_cxkk(&mut self, register: u8, value: u8) -> NextAddress {
        let random: u8 = random();
        self.register_set[register as usize] = value & random;

        NextAddress::DefaultIncrement
    }

    /// Dxyn Draw n byte sprite at location
    fn drw_dxyn(&mut self, x: u8, y: u8, size: u8) -> NextAddress {
        let start_location = self.register_i as usize;
        let end_location = start_location + (size as usize);
        let mem_read = &self.ram[start_location..end_location];

        let mut collide_flag = false;

        for row in 0..mem_read.len() {
            let sprite_row = mem_read[row];

            for i in 0..8 {
                let display_value = sprite_row >> (7 - i) & 0x01;

                if display_value == 1 {
                    let xx = (x + i) % 64;
                    let yy = (y as usize + row) % 32;
                    let display_location = yy * 64 + (xx as usize);

                    let current_display_value = self.internal_display[display_location];

                    if current_display_value == 1 {
                        collide_flag = true;
                    }

                    self.internal_display[display_location] = display_value ^ current_display_value;
                }
            }
        }

        self.register_set[0xF] = if collide_flag { 1 } else { 0 };

        NextAddress::DefaultIncrement
    }

    /// Ex9E Skip if key x pressed
    fn skk_ex9e(&self, x: u8, keys: KeyboardSet) -> NextAddress {
        if keys[x as usize] {
            NextAddress::Increment(4)
        } else {
            NextAddress::DefaultIncrement
        }
    }

    /// ExA1 Skip if key x not pressed
    fn sknk_exa1(&self, x: u8, keys: KeyboardSet) -> NextAddress {
        if !keys[x as usize] {
            NextAddress::Increment(4)
        } else {
            NextAddress::DefaultIncrement
        }
    }

    /// Fx07 Load delay timer into x
    fn ldd_fx07(&mut self, x: u8) -> NextAddress {
        self.register_set[x as usize] = self.delay_timer;

        NextAddress::DefaultIncrement
    }

    /// Fx0A Stop execution until key any key is pressed
    fn ldk_fx0a(&mut self, x: u8, keys: KeyboardSet) -> NextAddress {
        if let Some(k) = keys.iter().position(|k| k == &true) {
            self.register_set[x as usize] = k as u8;
            NextAddress::DefaultIncrement
        } else {
            NextAddress::NoIncrement
        }
    }

    /// Fx15 Load x into delay timer
    fn ldd_fx15(&mut self, x: u8) -> NextAddress {
        self.delay_timer = self.register_set[x as usize];

        NextAddress::DefaultIncrement
    }

    /// Fx18 Load x into sound timer
    fn lds_fx18(&mut self, x: u8) -> NextAddress {
        self.sound_timer = self.register_set[x as usize];

        NextAddress::DefaultIncrement
    }

    /// Fx1E Add register I and x and store in I
    fn adi_fx1e(&mut self, x: u8) -> NextAddress {
        self.register_i.add_assign(self.register_set[x as usize] as u16);

        NextAddress::DefaultIncrement
    }

    /// Fx29 Set I to the location of sprite for digit x
    fn ldi_fx29(&mut self, x: u8) -> NextAddress {
        self.register_i = (self.register_set[x as usize] as u16) * 5;

        NextAddress::DefaultIncrement
    }

    /// Fx33 Split up register x, load 100s in I, 10s in I + 1, 1s in I + 2
    fn str_fx33(&mut self, x: u8) -> NextAddress {
        let vx = self.register_set[x as usize];

        self.ram[self.register_i as usize] = vx / 100;
        self.ram[self.register_i as usize + 1] = (vx % 100) / 10;
        self.ram[self.register_i as usize + 2] = vx % 10;

        NextAddress::DefaultIncrement
    }

    /// Fx55 Store values of registers up to x in ram
    fn str_fx55(&mut self, x: u8) -> NextAddress {
        let register_i = self.register_i as usize;
        for register in 0..(x as usize) + 1 {
            self.ram[register_i + register] = self.register_set[register];
        }

        NextAddress::DefaultIncrement
    }

    /// Fx65 Load values of registers up to x from ram
    fn ldr_fx65(&mut self, x: u8) -> NextAddress {
        let register_i = self.register_i as usize;
        for register in 0..(x as usize) + 1 {
            self.register_set[register] = self.ram[register_i + register];
        }

        NextAddress::DefaultIncrement
    }
}