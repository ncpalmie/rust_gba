//! ARM7TDMI CPU emulation

pub mod registers;
pub mod arm;
pub mod thumb;
pub mod instructions;

use crate::bus::Bus;
pub use registers::Registers;

pub struct Cpu {
    pub registers: Registers;
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let _pc = self.registers.get_pc();
    }

    pub fn reset(&mut self) {
        self.registers.reset();
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new();
    }
}