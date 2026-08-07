//! Memory bus (routes memory reads and writes to appropriate regions)

use log::warn;

pub struct Bus {
    bios: [u8; 16384], // 16 KB BIOS
    ewram: [u8; 262144], // 256 KB external work RAM
    iwram: [u8; 32768], // 32 KB internal work RAM
    vram: [u8; 98304], // 32 KB video RAM
    oam: [u8; 1024], // 1 KB OAM (object attribute memory)
    pram: [u8; 1024], // 1 KB palette RAM
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            bios: [0; 16384],
            ewram: [0; 262144],
            iwram: [0; 32768],
            vram: [0; 98304],
            oam: [0; 1024],
            pram: [0; 1024],
        }
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        warn!("read_byte not implemented yet");
        return 0;
    }

    pub fn write_byte(&self, address: u32, value: u8) {
        warn!("write_byte not implemented yet");
    }

    pub fn read_halfword(&self, address: u32) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;
        return (high << 8) | low;
    }

    pub fn write_halfword(&self, address: u32, value: u16) {
        self.write_byte(address, (value & 0xff) as u8);
        self.write_byte(address + 1, ((value >> 8) & 0xff) as u8)
    }

    pub fn read_word(&self, address: u32) -> u32 {
        let low = self.read_halfword(address) as u32;
        let high = self.read_halfword(address + 2) as u32;
        (high << 16) | low
    }

    pub fn write_word(&self, address: u32, value: u32) {
        self.write_halfword(address, (value & 0xffff) as u16);
        self.write_halfword(address + 2, ((value >> 16) & 0xffff) as u16);
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
