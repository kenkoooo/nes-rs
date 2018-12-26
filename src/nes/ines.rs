use std::io::Read;

pub struct NesFileHeader {
    pub magic: u32,
    pub num_prg: u8,
    pub num_chr: u8,
    pub control1: u8,
    pub control2: u8,
    num_ram: u8,
    padding: [u8; 7],
}

impl NesFileHeader {
    pub fn new(header: [u8; 16]) -> Self {
        let mut magic_buf: [u8; 4] = [0; 4];
        magic_buf.copy_from_slice(&header[0..4]);
        let magic = u32::from_le_bytes(magic_buf);

        let mut padding: [u8; 7] = [0; 7];
        padding.copy_from_slice(&header[9..16]);
        NesFileHeader {
            magic: magic,
            num_prg: header[4],
            num_chr: header[5],
            control1: header[6],
            control2: header[7],
            num_ram: header[8],
            padding: padding,
        }
    }
}
