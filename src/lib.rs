pub mod bus;
pub mod cpu;
pub mod memory;
pub mod ppu;
pub mod apu;
pub mod cartridge;
pub mod utils;

pub use cpu::Cpu;
pub use bus::Bus;

// GBA system constants
pub mod constants {
    pub const SCREEN_WIDTH: usize = 240;
    pub const SCREEN_HEIGHT: usize = 160;
    pub const CPU_FREQUENCY: u32 = 16_777_216; // 16.78 MHz
    pub const CYCLES_PER_FRAME: u32 = 280_896; // 16.78 MHz / 59.73 Hz
}
