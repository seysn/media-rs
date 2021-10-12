use std::fs::File;
use std::io::{BufWriter, Write};

pub struct BMP {
    bitmap_file_header: BitmapFileHeader
}

pub struct BitmapFileHeader {
    signature: u16,
    size: u32,
    reserved1: u16,
    reserved2: u16,
    offset: u32
}

impl Default for BitmapFileHeader {
    fn default() -> Self {
        Self {
            signature: 0x424d,
            size: 70,
            reserved1: 0,
            reserved2: 0,
            offset: 54,
        }
    }
}

impl BMP {
    pub fn new(width: usize, height: usize) -> BMP {
        BMP { bitmap_file_header: BitmapFileHeader::default() }
    }

    pub fn write(self, filename: String) {
        let file = File::open(filename).unwrap();
        // TODO
    }
}
