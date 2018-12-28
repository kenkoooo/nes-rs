use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::nes::ines::NesFileHeader;

pub struct Cartridge {
    pub prg: Vec<u8>,
    chr: Vec<u8>,
    sram: [u8; 0x2000],
    pub mapper: u8,
    mirror: u8,
    pub battery: u8,
}

pub fn load_nes_file(path: &str) -> io::Result<Cartridge> {
    let mut f = File::open(path)?;

    let mut header: [u8; 16] = [0; 16];
    f.read_exact(&mut header)?;

    let header = NesFileHeader::new(header);

    // mapper type
    let mapper1 = header.control1 >> 4;
    let mapper2 = header.control2 >> 4;
    let mapper = mapper1 | mapper2 << 4;

    // mirroring type
    let mirror1 = header.control1 & 1;
    let mirror2 = (header.control1 >> 3) & 1;
    let mirror = mirror1 | mirror2 << 1;

    // battery-backed RAM
    let battery = (header.control1 >> 1) & 1;

    if header.control1 & 4 == 4 {
        let mut trainer = [0; 512];
        f.read_exact(&mut trainer)?;
    }

    let prg_size = (header.num_prg as usize) * 16384;
    let mut prg = vec![0; prg_size];
    f.read_exact(&mut prg[..])?;

    let chr_size = (header.num_chr as usize) * 8192;
    let mut chr = vec![0; chr_size];
    f.read_exact(&mut chr[..])?;

    if header.num_chr == 0 {
        chr = vec![0; 8192];
    }
    Ok(Cartridge::new(prg, chr, mapper, mirror, battery))
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
