use crate::constants::*;
use crate::Chip8Emulator;
use crate::macros::mask;

impl Chip8Emulator {
    // 0x00E0
    pub(crate) fn clear_screen(&mut self, _instruction: u16) {
        self.display_ram = [0; DISPLAY_WIDTH * DISPLAY_HEIGHT + 1];
    }
    // 0x00EE
    pub(crate) fn return_from_subroutine(&mut self, _instruction: u16) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }

    // 0x1nnn
    pub(crate) fn jmp_addr(&mut self, instruction: u16) {
        let addr = mask!(instruction, 123);
        self.program_counter = addr as usize;
    }

    // 0x2nnn
    pub(crate) fn call_addr(&mut self, _instruction: u16) {
        self.stack[self.stack_pointer] = self.program_counter as u16;
        self.stack_pointer += 1;
    }

    // 0x3xkk
    pub(crate) fn skip_register_eq(&mut self, instruction: u16) {
        let (x, kk) = mask!(instruction, 1, 23);
        if self.v_registers[x] == kk {
            self.program_counter += 2;
        }
    }

    // 0x4xkk
    pub(crate) fn skip_register_ne(&mut self, instruction: u16) {
        let (x, kk) = mask!(instruction, 1, 23);
        if self.v_registers[x] != kk {
            self.program_counter += 2;
        }
    }

    // 0x5xy0
    pub(crate) fn skip_registers_eq(&mut self, instruction: u16) {
        let (x, y) = mask!(instruction, 1, 2);
        if self.v_registers[x] == self.v_registers[y] {
            self.program_counter += 2;
        }
    }

    // 0x6xkk
    pub(crate) fn set_register(&mut self, instruction: u16) {
        let (x, kk) = mask!(instruction, 1, 23);
        self.v_registers[x] = kk;
    }

    // 0x7xkk
    pub(crate) fn add_register(&mut self, instruction: u16) {
        let (x, kk) = mask!(instruction, 1, 23);
        let (v, _) = self.v_registers[x].overflowing_add(kk);
        self.v_registers[x] = v;
    }

    // 0x8xy0
    pub(crate) fn load_xy(&mut self, instruction: u16) {
        let (x, y) = mask!(instruction, 1, 2);
        self.v_registers[x] = self.v_registers[y];
    }
    // 0x8xy1
    pub(crate) fn or_xy(&mut self, instruction: u16) {
        let (x, y) = mask!(instruction, 1, 2);
        self.v_registers[x] |= self.v_registers[y];
    }
    // 0x8xy2
    pub(crate) fn and_xy(&mut self, instruction: u16) {
        let (x, y) = mask!(instruction, 1, 2);
        self.v_registers[x] &= self.v_registers[y];
    }
    // 0x8xy3
    pub(crate) fn xor_xy(&mut self, instruction: u16) {
        let (x, y) = mask!(instruction, 1, 2);
        self.v_registers[x] ^= self.v_registers[y];
    }
    // 0x8xy4
    pub(crate) fn add_xy(&mut self, instruction: u16) {
        let (x, y) = mask!(instruction, 1, 2);
        let (v, overflow) = self.v_registers[x].overflowing_add(self.v_registers[y]);
        self.v_registers[0xF] = if overflow { 1 } else { 0 };
        self.v_registers[x] = v;
    }
    // 0x8xy5
    pub(crate) fn sub_xy(&mut self, instruction: u16) {
        let (x, y) = mask!(instruction, 1, 2);
        let (v, _) = self.v_registers[x].overflowing_sub(self.v_registers[y]);
        self.v_registers[0xF] = if self.v_registers[x] > self.v_registers[y] { 1 } else { 0 };
        self.v_registers[x] = v;
    }
    // 0x8xy6
    pub(crate) fn shr_xy(&mut self, instruction: u16) {
        let x = mask!(1, instruction);
        self.v_registers[0xF] = self.v_registers[x] & 0x01;
        self.v_registers[x] >>= 1;
    }
    // 0x8xy7
    pub(crate) fn subn_xy(&mut self, instruction: u16) {
        let (x, y) = mask!(instruction, 1, 2);
        let (v, _) = self.v_registers[x].overflowing_sub(self.v_registers[y]);
        self.v_registers[0xF] = if self.v_registers[x] < self.v_registers[y] { 1 } else { 0 };
        self.v_registers[x] = v;
    }
    // 0x8xyE
    pub(crate) fn shl_xy(&mut self, instruction: u16) {
        let x = mask!(1, instruction);
        self.v_registers[0xF] = if (self.v_registers[x] & 0x80) > 0 { 1 } else { 0 };
        self.v_registers[x] <<= 1;
    }

    // 0x9xy0
    pub(crate) fn skip_registers_ne(&mut self, instruction: u16) {
        let (x, y) = mask!(instruction, 1, 2);
        if self.v_registers[x] != self.v_registers[y] {
            self.program_counter += 2;
        }
    }

    // 0xAnnn
    pub(crate) fn set_i_register(&mut self, instruction: u16) {
        let addr = mask!(123, instruction);
        self.i_register = addr;
    }

    // 0xBnnn
    pub(crate) fn jmp_v0(&mut self, instruction: u16) {
        let addr = mask!(123, instruction);
        self.program_counter = self.v_registers[0x0] as usize + addr as usize;
    }

    // 0xCxkk
    pub(crate) fn rand_byte(&mut self, instruction: u16) {
        let (x, kk) = mask!(instruction, 1, 23);
        self.v_registers[x] = rand::random::<u8>() & kk;
    }

    // 0xDxyn
    pub(crate) fn draw_sprite(&mut self, instruction: u16) {
        let (x, y, n) = mask!(instruction, 1, 2, 3);
        let x_pos = self.v_registers[x] as usize % DISPLAY_WIDTH;
        let y_pos = self.v_registers[y] as usize % DISPLAY_HEIGHT;

        self.v_registers[0xF] = 0;

        for row in 0..n {
            let byte = self.memory[self.i_register as usize + row];

            for col in 0..8 {
                let mut index = (y_pos + row) * DISPLAY_WIDTH + (x_pos + col);
                if index >= 2048 { index = 2047 };

                let pixel = byte & (0x80 >> col);
                let screen_pixel = &mut self.display_ram[index];

                if pixel > 0 {
                    if *screen_pixel == 0xFF {
                        self.v_registers[0xF] = 1;
                    }

                    *screen_pixel ^= 0xFF;
                }
            }
        }
    }

    // 0xEx9E
    pub(crate) fn skip_vx_key(&mut self, instruction: u16) {
        let x = mask!(1, instruction);
        if self.key_flags[self.v_registers[x] as usize] {
            self.program_counter += 2;
        }
    }
    // 0xExA1
    pub(crate) fn nskip_vx_key(&mut self, instruction: u16) {
        let x = mask!(1, instruction);
        if !self.key_flags[self.v_registers[x] as usize] {
            self.program_counter += 2;
        }
    }

    // 0xFx07
    pub(crate) fn load_delay(&mut self, instruction: u16) {
        let x = mask!(1, instruction);
        self.v_registers[x] = self.delay_register;
    }
    // 0xFx0A
    pub(crate) fn wait_key(&mut self, instruction: u16) {
        let x = mask!(1, instruction);
        let result = self.key_flags.iter().enumerate().find(|(_, v)| **v);
        if let Some((i, _)) = result {
            self.v_registers[x] = i as u8;
        } else { self.program_counter -= 2; }
    }
    // 0xFx15
    pub(crate) fn set_delay(&mut self, instruction: u16) {
        let x = mask!(1, instruction);
        self.delay_register = self.v_registers[x];
    }
    // 0xFx18
    pub(crate) fn set_sound(&mut self, instruction: u16) {
        let x = mask!(1, instruction);
        self.sound_register = self.v_registers[x];
    }
    // 0xFx1E
    pub(crate) fn set_add_i_register(&mut self, instruction: u16) {
        let x = mask!(1, instruction);
        self.i_register += self.v_registers[x] as u16;
    }
    // 0xFx29
    pub(crate) fn set_sprite_location(&mut self, instruction: u16) {
        let x = mask!(1, instruction);
        self.i_register = self.v_registers[x] as u16 * 5;
    }
    // 0xFx33
    pub(crate) fn store_bcd(&mut self, instruction: u16) {
        let x = mask!(1, instruction);
        let mut value = self.v_registers[x];
        self.memory[self.i_register as usize + 2] = value % 10;
        value /= 10;
        self.memory[self.i_register as usize + 1] = value % 10;
        value /= 10;
        self.memory[self.i_register as usize] = value % 10;
    }
    // 0xFx55
    pub(crate) fn store_registers(&mut self, _instruction: u16) {
        for (i, register) in self.v_registers.iter().enumerate() {
            self.memory[self.i_register as usize + i] = *register;
        }
    }
    // 0xFx65
    pub(crate) fn load_registers(&mut self, _instruction: u16) {
        for (i, register) in self.v_registers.iter_mut().enumerate() {
            *register = self.memory[self.i_register as usize + i];
        }
    }
}
