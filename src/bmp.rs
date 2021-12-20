use std::fs::File;

use crate::utils::{Endianness, read_u16, read_u32};

#[derive(Debug)]
pub struct BMP {
    bitmap_file_header: BitmapFileHeader,
    dib_header: DIBHeader
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
    file_size: u32,
    reserved1: u16,
    reserved2: u16,
    offset: u32
}

#[derive(Debug)]
pub struct DIBHeader {
    dib_header_size: u32,
    width: u32,
    height: u32,
    planes: u16,
    bits_per_pixel: u16,
    compression: u32,
    image_size: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    colors: u32,
    important_colors: u32,
}

impl BMP {
    pub fn read(filename: String) -> Result<BMP, String> {
        let mut file = File::open(filename).unwrap();

        let signature = Signature::from_u16(read_u16(&mut file, Endianness::BigEndian)?)?;
        let file_size = read_u32(&mut file, Endianness::LittleEndian)?;
        let reserved1 = read_u16(&mut file, Endianness::BigEndian)?;
        let reserved2 = read_u16(&mut file, Endianness::BigEndian)?;
        let offset = read_u32(&mut file, Endianness::LittleEndian)?;
        let bitmap_file_header = BitmapFileHeader{
            signature,
            file_size,
            reserved1,
            reserved2,
            offset
        };

        let dib_header_size = read_u32(&mut file, Endianness::LittleEndian)?;
        let width = read_u32(&mut file, Endianness::LittleEndian)?;
        let height = read_u32(&mut file, Endianness::LittleEndian)?;
        let planes = read_u16(&mut file, Endianness::LittleEndian)?;
        let bits_per_pixel = read_u16(&mut file, Endianness::LittleEndian)?;
        let compression = read_u32(&mut file, Endianness::LittleEndian)?;
        let image_size = read_u32(&mut file, Endianness::LittleEndian)?;
        let horizontal_resolution = read_u32(&mut file, Endianness::LittleEndian)?;
        let vertical_resolution = read_u32(&mut file, Endianness::LittleEndian)?;
        let colors = read_u32(&mut file, Endianness::LittleEndian)?;
        let important_colors = read_u32(&mut file, Endianness::LittleEndian)?;
        let dib_header = DIBHeader { 
            dib_header_size,
            width,
            height,
            planes,
            bits_per_pixel,
            compression,
            image_size,
            horizontal_resolution,
            vertical_resolution,
            colors,
            important_colors
        };

        Ok(BMP { bitmap_file_header, dib_header })
    }
}
