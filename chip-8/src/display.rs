use std::fmt::Display;
use crate::Chip8Emulator;

impl Display for Chip8Emulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use rhexdump::prelude::*;

        let config = RhexdumpBuilder::default()
            .hide_duplicate_lines(true)
            .build_string();

        let v_regs = self.v_registers.iter()
            .enumerate()
            .map(|(i, v)| format!("{} 0x{:X}", i, *v))
            .collect::<Vec<_>>()
            .join(" / ");

        let keys = self.key_flags.iter()
            .enumerate()
            .map(|(i, v)| format!("{:X} {}", i, if *v { "on" } else { "off" }))
            .collect::<Vec<_>>()
            .join(" / ");

        let stack = self.stack.iter()
            .map(|v| format!("0x{:X}", *v))
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(f, "Emulator (Chip-8)")?;
        writeln!(
            f, 
            "Registers:\n\tI 0x{:X} | Sound {}ms | Delay {}ms\n\tV {}", 
            self.i_register, self.sound_register, self.delay_register, v_regs,
        )?;
        writeln!(f, "Keys: [{}]", keys)?;
        writeln!(f, "Program Counter: 0x{:X} | Stack Pointer: 0x{:X}", self.program_counter, self.stack_pointer)?;
        writeln!(f, "Stack: [{stack}]")?;
        writeln!(f, "Memory:\n{}", config.hexdump_bytes(self.memory))?;
        writeln!(f, "Display:\n{}", config.hexdump_bytes(self.display_ram))?;

        Ok(())
    }
}
