mod chip_8;
mod cpu;
mod display;
mod memory;

fn main() {
    // TODO: wrap this stuff in a CHIP_8 module
    let mut instance = chip_8::CHIP_8::new();
    instance.run();
}
