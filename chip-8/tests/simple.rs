use chip_8::Chip8Emulator;

#[test]
fn simple_emulator() {
    let test = std::fs::read("./roms/BC_test.ch8").unwrap();
    let emulator = Chip8Emulator::new(test.as_slice());
    println!("{emulator}");
}

#[test]
fn test_run() {
    let test = std::fs::read("./roms/BC_test.ch8").unwrap();
    let mut emulator = Chip8Emulator::new(test.as_slice());
    for _ in 0..255 {
        emulator.tick();
    }
    println!("{emulator}");
}
