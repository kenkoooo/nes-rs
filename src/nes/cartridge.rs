pub struct Cartridge {
    prg: Vec<u8>,
    chr: Vec<u8>,
    sram: [u8; 0x2000],
    mapper: u8,
    mirror: u8,
    battery: u8,
}

impl Cartridge {
    pub fn new(prg: Vec<u8>, chr: Vec<u8>, mapper: u8, mirror: u8, battery: u8) -> Cartridge {
        Cartridge {
            prg: prg,
            chr: chr,
            sram: [0; 0x2000],
            mapper: mapper,
            mirror: mirror,
            battery: battery,
        }
    }

    pub fn save() {
        unimplemented!()
    }

    pub fn load() {
        unimplemented!()
    }
}
