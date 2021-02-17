pub mod constants;
pub mod instructions;
pub mod cpu;

use constants::*;

fn main() {
    let cpu = cpu::Cpu::new();
}
