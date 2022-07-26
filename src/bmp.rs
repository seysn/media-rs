use std::{
    fmt::Debug,
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use crate::image::{make_buffer, Pixel};
use crate::utils::{read_u16, read_u32, Endianness};

pub struct BMP {
    bitmap_file_header: BitmapFileHeader,
    dib_header: DIBHeader,
    content: Vec<Pixel>,
}

impl Debug for BMP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BMP")
            .field("bitmap_file_header", &self.bitmap_file_header)
            .field("dib_header", &self.dib_header)
            .field("content", &format!("[{} pixels]", self.content.len()))
            .finish()
    }
}

#[derive(Debug, PartialEq)]
enum Signature {
    BM = 0x424d, // Windows 3.1x, 95, NT, ... etc.
    BA = 0x4241, // OS/2 struct bitmap array
    CI = 0x4349, // OS/2 struct color icon
    CP = 0x4350, // OS/2 const color pointer
    IC = 0x4943, // OS/2 struct icon
    PT = 0x5054, // OS/2 pointer
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
            _ => return Err(format!("invalid signature : {}", value)),
        })
    }
}

// const SIGNATURES: &'static [Signature; 6] = &[Signature::BM, Signature::BA, Signature::CI, Signature::CP, Signature::IC, Signature::PT];

#[derive(Debug)]
#[allow(dead_code)]
pub struct BitmapFileHeader {
    signature: Signature,
    file_size: u32,
    reserved1: u16,
    reserved2: u16,
    offset: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
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

        let bitmap_file_header = BitmapFileHeader {
            signature: Signature::from_u16(read_u16(&mut file, Endianness::BigEndian)?)?,
            file_size: read_u32(&mut file, Endianness::LittleEndian)?,
            reserved1: read_u16(&mut file, Endianness::BigEndian)?,
            reserved2: read_u16(&mut file, Endianness::BigEndian)?,
            offset: read_u32(&mut file, Endianness::LittleEndian)?,
        };

        let dib_header = DIBHeader {
            dib_header_size: read_u32(&mut file, Endianness::LittleEndian)?,
            width: read_u32(&mut file, Endianness::LittleEndian)?,
            height: read_u32(&mut file, Endianness::LittleEndian)?,
            planes: read_u16(&mut file, Endianness::LittleEndian)?,
            bits_per_pixel: read_u16(&mut file, Endianness::LittleEndian)?,
            compression: read_u32(&mut file, Endianness::LittleEndian)?,
            image_size: read_u32(&mut file, Endianness::LittleEndian)?,
            horizontal_resolution: read_u32(&mut file, Endianness::LittleEndian)?,
            vertical_resolution: read_u32(&mut file, Endianness::LittleEndian)?,
            colors: read_u32(&mut file, Endianness::LittleEndian)?,
            important_colors: read_u32(&mut file, Endianness::LittleEndian)?,
        };

        // Jumping on the location of image data
        file.seek(SeekFrom::Start(bitmap_file_header.offset as u64))
            .unwrap();

        // Image size is not always specified, so we make sure we get a value
        let image_size = if dib_header.image_size != 0 {
            dib_header.image_size
        } else {
            dib_header.width * dib_header.height * dib_header.bits_per_pixel as u32 / 8
        };

        let mut buffer = vec![0; image_size as usize];
        file.read_exact(&mut buffer).unwrap();

        let content = make_buffer(
            &buffer,
            dib_header.bits_per_pixel as usize,
            dib_header.width as usize,
        );

        Ok(BMP {
            bitmap_file_header,
            dib_header,
            content,
        })
    }
}
