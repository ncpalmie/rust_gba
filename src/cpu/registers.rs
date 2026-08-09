//! ARM7TDMI registers

use bitflags::bitflags;

/// CPU mode (bottom 5 bits of CPSR)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    User = 0b10000,
    Fiq = 0b10001,
    Irq = 0b10010,
    Supervisor = 0b10011,
    Abort = 0b10111,
    Undefined = 0b11011,
    System = 0b11111,
}

bitflags! {
    /// CPU status flags (CPSR/SPSR)
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusFlags: u32 {
        // Condition flags
        const NEGATIVE = 1 << 31; // N flag
        const ZERO = 1 << 30; // Z flag
        const CARRY = 1 << 29; // C flag
        const OVERFLOW = 1 << 28; // V flag

        const IRQ_DISABLE = 1 << 7; // I flag
        const FIQ_DISABLE = 1 << 6; // F flag
        const THUMB = 1 << 5; // T flag (thumb mode 16 or 32 bit instructions)
    }
}

pub struct Registers {
    /// General purpose registers R0-R15
    /// R13 = SP (stack pointer)
    /// R14 = LR (link register)
    /// R15 = PC (program counter)
    pub r: [u32; 16],

    // Current program status register
    pub cpsr: u32,
}

impl Registers {
    pub fn new() -> Self {
        let mut regs = Registers {
            r: [0; 16],
            cpsr: Mode::System as u32,
        };
        regs.reset();
        regs
    }

    pub fn reset(&mut self) {
        self.r = [0; 16];
        // GBA BIOS sets PC to 0x08000000 (ROM entry point) after initialization
        self.set_pc(0x08000000);
        self.cpsr = Mode::System as u32;
    }

    pub fn get_pc(&self) -> u32 {
        self.r[15]
    }

    pub fn set_pc(&mut self, value: u32) {
        self.r[15] = value;
    }

    pub fn is_thumb(&self) -> bool {
        (self.cpsr & StatusFlags::THUMB.bits()) != 0
    }

    pub fn get_mode(&self) -> Mode {
        match self.cpsr & 0x1f {
            0b10000 => Mode::User,
            0b10001 => Mode::Fiq,
            0b10010 => Mode::Irq,
            0b10011 => Mode::Supervisor,
            0b10111 => Mode::Abort,
            0b11011 => Mode::Undefined,
            0b11111 => Mode::System,
            _ => Mode::System, // default to System
        }
    }

    pub fn get_n(&self) -> bool {
        (self.cpsr & StatusFlags::NEGATIVE.bits()) != 0
    }

    pub fn set_n(&mut self, value: bool) {
        if value {
            self.cpsr |= StatusFlags::NEGATIVE.bits();
        } else {
            self.cpsr &= !StatusFlags::NEGATIVE.bits();
        }
    }

    pub fn get_z(&self) -> bool {
        (self.cpsr & StatusFlags::ZERO.bits()) != 0
    }

    pub fn set_z(&mut self, value: bool) {
        if value {
            self.cpsr |= StatusFlags::ZERO.bits();
        } else {
            self.cpsr &= !StatusFlags::ZERO.bits();
        }
    }

    pub fn get_c(&self) -> bool {
        (self.cpsr & StatusFlags::CARRY.bits()) != 0
    }

    pub fn set_c(&mut self, value: bool) {
        if value {
            self.cpsr |= StatusFlags::CARRY.bits();
        } else {
            self.cpsr &= !StatusFlags::CARRY.bits();
        }
    }

    pub fn get_v(&self) -> bool {
        (self.cpsr & StatusFlags::OVERFLOW.bits()) != 0
    }

    pub fn set_v(&mut self, value: bool) {
        if value {
            self.cpsr |= StatusFlags::OVERFLOW.bits();
        } else {
            self.cpsr &= !StatusFlags::OVERFLOW.bits();
        }
    }

    pub fn set_thumb(&mut self, value: bool) {
        if value {
            self.cpsr |= StatusFlags::THUMB.bits();
        } else {
            self.cpsr &= !StatusFlags::THUMB.bits();
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}
