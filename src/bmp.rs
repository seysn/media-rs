use std::fs::File;

use crate::utils::{Endianness, read_u16, read_u32};

#[derive(Debug)]
pub struct BMP {
    bitmap_file_header: BitmapFileHeader
}

#[derive(Debug, PartialEq)]
enum Signature {
    BM = 0x424d,    // Windows 3.1x, 95, NT, ... etc.
    BA = 0x4241,    // OS/2 struct bitmap array
    CI = 0x4349,    // OS/2 struct color icon
    CP = 0x4350,    // OS/2 const color pointer
    IC = 0x4943,    // OS/2 struct icon
    PT = 0x5054,    // OS/2 pointer
}

impl Signature {
    fn from_u16(value: u16) -> Result<Signature, String> {
        Ok(match value {
            0x424d => Signature::BM,
            0x4241 => Signature::BA,
            0x4349 => Signature::CI,
            0x4350 => Signature::CP,
            0x4943 => Signature::IC,
            0x5054 => Signature::PT,
            _ => return Err(format!("invalid signature : {}", value))
        })
    }
}

// const SIGNATURES: &'static [Signature; 6] = &[Signature::BM, Signature::BA, Signature::CI, Signature::CP, Signature::IC, Signature::PT];

#[derive(Debug)]
pub struct BitmapFileHeader {
    signature: Signature,
    size: u32,
    reserved1: u16,
    reserved2: u16,
    offset: u32
}

impl Default for BitmapFileHeader {
    fn default() -> Self {
        Self {
            signature: Signature::BM,
            size: 70,
            reserved1: 0,
            reserved2: 0,
            offset: 54,
        }
    }
}

impl BMP {
    pub fn read(filename: String) -> Result<BMP, String> {
        let mut file = File::open(filename).unwrap();

        let signature = Signature::from_u16(read_u16(&mut file, Endianness::BigEndian)?)?;
        let size = read_u32(&mut file, Endianness::LittleEndian)?;
        let reserved1 = read_u16(&mut file, Endianness::BigEndian)?;
        let reserved2 = read_u16(&mut file, Endianness::BigEndian)?;
        let offset = read_u32(&mut file, Endianness::LittleEndian)?;

        let bitmap_file_header = BitmapFileHeader{ signature, size, reserved1, reserved2, offset};

        Ok(BMP { bitmap_file_header })
    }
}
