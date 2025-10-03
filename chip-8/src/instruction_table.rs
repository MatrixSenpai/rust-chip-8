use crate::Chip8Emulator;
use crate::macros::mask;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Lookup {
    Value(fn(&mut Chip8Emulator, u16)),
    Table(fn(&mut Chip8Emulator, u16) -> Self),
}
impl Lookup {
    pub(crate) fn resolve(&self, e: &mut Chip8Emulator, i: u16) {
        match self {
            Self::Value(v) => v(e, i),
            Self::Table(t) => t(e, i).resolve(e, i)
        }
    }
}

pub const MAIN_INSTRUCTION_TABLE: [Lookup; 16] = make_main_table();
pub const TABLE_0: [Lookup; 16] = make_0_table();
pub const TABLE_8: [Lookup; 16] = make_8_table();
pub const TABLE_E: [Lookup; 16] = make_e_table();
pub const TABLE_F: [Lookup; 102] = make_f_table();

const fn make_main_table() -> [Lookup; 16] {
    let mut table = [Lookup::Value(Chip8Emulator::noop); 16];

    table[0x0] = Lookup::Table(lookup_in_0);
    table[0x1] = Lookup::Value(Chip8Emulator::jmp_addr);
    table[0x2] = Lookup::Value(Chip8Emulator::call_addr);
    table[0x3] = Lookup::Value(Chip8Emulator::skip_register_eq);
    table[0x4] = Lookup::Value(Chip8Emulator::skip_register_ne);
    table[0x5] = Lookup::Value(Chip8Emulator::skip_registers_eq);
    table[0x6] = Lookup::Value(Chip8Emulator::set_register);
    table[0x7] = Lookup::Value(Chip8Emulator::add_register);
    table[0x8] = Lookup::Table(lookup_in_8);
    table[0x9] = Lookup::Value(Chip8Emulator::skip_registers_ne);
    table[0xA] = Lookup::Value(Chip8Emulator::set_i_register);
    table[0xB] = Lookup::Value(Chip8Emulator::jmp_v0);
    table[0xC] = Lookup::Value(Chip8Emulator::rand_byte);
    table[0xD] = Lookup::Value(Chip8Emulator::draw_sprite);
    table[0xE] = Lookup::Table(lookup_in_e);
    table[0xF] = Lookup::Table(lookup_in_f);

    table
}

const fn make_0_table() -> [Lookup; 16] {
    let mut table = [Lookup::Value(Chip8Emulator::noop); 16];

    table[0x0] = Lookup::Value(Chip8Emulator::clear_screen);
    table[0xE] = Lookup::Value(Chip8Emulator::return_from_subroutine);

    table
}
const fn make_8_table() -> [Lookup; 16] {
    let mut table = [Lookup::Value(Chip8Emulator::noop); 16];

    table[0x0] = Lookup::Value(Chip8Emulator::load_xy);
    table[0x1] = Lookup::Value(Chip8Emulator::or_xy);
    table[0x2] = Lookup::Value(Chip8Emulator::and_xy);
    table[0x3] = Lookup::Value(Chip8Emulator::xor_xy);
    table[0x4] = Lookup::Value(Chip8Emulator::add_xy);
    table[0x5] = Lookup::Value(Chip8Emulator::sub_xy);
    table[0x6] = Lookup::Value(Chip8Emulator::shr_xy);
    table[0x7] = Lookup::Value(Chip8Emulator::subn_xy);
    table[0xE] = Lookup::Value(Chip8Emulator::shl_xy);

    table
}
const fn make_e_table() -> [Lookup; 16] {
    let mut table = [Lookup::Value(Chip8Emulator::noop); 16];

    table[0xE] = Lookup::Value(Chip8Emulator::skip_vx_key);
    table[0x1] = Lookup::Value(Chip8Emulator::nskip_vx_key);

    table
}
const fn make_f_table() -> [Lookup; 102] {
    let mut table = [Lookup::Value(Chip8Emulator::noop); 102];

    table[0x07] = Lookup::Value(Chip8Emulator::load_delay);
    table[0x0A] = Lookup::Value(Chip8Emulator::wait_key);
    table[0x15] = Lookup::Value(Chip8Emulator::set_delay);
    table[0x18] = Lookup::Value(Chip8Emulator::set_sound);
    table[0x1E] = Lookup::Value(Chip8Emulator::set_add_i_register);
    table[0x29] = Lookup::Value(Chip8Emulator::set_sprite_location);
    table[0x33] = Lookup::Value(Chip8Emulator::store_bcd);
    table[0x55] = Lookup::Value(Chip8Emulator::store_registers);
    table[0x65] = Lookup::Value(Chip8Emulator::load_registers);

    table
}

const fn lookup_in_0(_e: &mut Chip8Emulator, instruction: u16) -> Lookup {
    TABLE_0[mask!(instruction, 3)]
}
const fn lookup_in_8(_e: &mut Chip8Emulator, instruction: u16) -> Lookup {
    TABLE_8[mask!(instruction, 3)]
}
const fn lookup_in_e(_e: &mut Chip8Emulator, instruction: u16) -> Lookup {
    TABLE_E[mask!(instruction, 3)]
}
const fn lookup_in_f(_e: &mut Chip8Emulator, instruction: u16) -> Lookup {
    TABLE_F[mask!(instruction, 23) as usize]
}

impl Chip8Emulator {
    fn noop(&mut self, instruction: u16) {
        println!("Noop called unexpectedly! {instruction:X?}");
    }
}
