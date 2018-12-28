use crate::nes::Cartridge;

pub struct Mapper4 {
    register: u8,
    registers: [u8; 8],
    prg_mode: u8,
    chr_mode: u8,
    prg_offsets: [i32; 4],
    chr_offsets: [i32; 8],
    reload: u8,
    counter: u8,
    irq_enable: bool,
}

impl Mapper4 {
    pub fn new(cartridge: &Cartridge) -> Self {
        let mut mapper = Mapper4 {
            register: 0,
            registers: [0; 8],
            prg_mode: 0,
            chr_mode: 0,
            prg_offsets: [0; 4],
            chr_offsets: [0; 8],
            reload: 0,
            counter: 0,
            irq_enable: false,
        };
        mapper.prg_offsets[0] = Self::prg_bank_offset(0, cartridge);
        mapper.prg_offsets[1] = Self::prg_bank_offset(1, cartridge);
        mapper.prg_offsets[2] = Self::prg_bank_offset(-2, cartridge);
        mapper.prg_offsets[3] = Self::prg_bank_offset(-1, cartridge);
        mapper
    }

    pub fn prg_bank_offset(mut index: i32, cartridge: &Cartridge) -> i32 {
        if index >= 0x80 {
            index -= 0x100;
        }
        index %= (cartridge.prg.len() / 0x2000) as i32;
        let mut offset = index * 0x2000;
        if offset < 0 {
            offset += cartridge.prg.len() as i32;
        }
        offset
    }
}
