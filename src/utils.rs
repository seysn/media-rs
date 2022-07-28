use std::{fs, io::Read};

use crate::error::{MediaResult, MediaError};

#[derive(PartialEq)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

pub fn read_u8(file: &mut fs::File) -> MediaResult<u8> {
    let mut buf = [0; 1];
    let res = file.read(&mut buf)?;

    if res < 1 {
        Err(MediaError::DecodingError("reached end of file".to_string()))
    } else {
        Ok(buf[0])
    }
}

pub fn read_u16(file: &mut fs::File, endianness: Endianness) -> MediaResult<u16> {
    let mut buf = [0; 2];
    let res = file.read(&mut buf)?;

    if res < 2 {
        Err(MediaError::DecodingError("reached end of file".to_string()))
    } else if endianness == Endianness::LittleEndian {
        Ok(((buf[1] as u16) << 8) + buf[0] as u16)
    } else {
        Ok(((buf[0] as u16) << 8) + buf[1] as u16)
    }
}

pub fn read_u32(file: &mut fs::File, endianness: Endianness) -> MediaResult<u32> {
    let mut buf = [0; 4];
    let res = file.read(&mut buf)?;

    if res < 4 {
        Err(MediaError::DecodingError("reached end of file".to_string()))
    } else if endianness == Endianness::LittleEndian {
        Ok(((buf[3] as u32) << 24)
            + ((buf[2] as u32) << 16)
            + ((buf[1] as u32) << 8)
            + buf[0] as u32)
    } else {
        Ok(((buf[0] as u32) << 24)
            + ((buf[1] as u32) << 16)
            + ((buf[2] as u32) << 8)
            + buf[3] as u32)
    }
}
