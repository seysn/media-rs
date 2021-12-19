use std::{fs, io::Read};

#[derive(PartialEq)]
pub enum Endianness {
    LittleEndian,
    BigEndian
}

pub fn read_u8(file: &mut fs::File) -> Result<u8, String> {
    let mut buf = [0; 1];
    let res = file.read(&mut buf);

    if res.is_err() {
        Err(res.unwrap_err().to_string())
    } else if res.unwrap() < 1 {
        Err("reached end of file".to_string())
    } else {
        Ok(buf[0])
    }
}

pub fn read_u16(file: &mut fs::File, endianness: Endianness) -> Result<u16, String> {
    let mut buf = [0; 2];
    let res = file.read(&mut buf);

    if res.is_err() {
        Err(res.unwrap_err().to_string())
    } else if res.unwrap() < 2 {
        Err("reached end of file".to_string())
    } else if endianness == Endianness::LittleEndian {
        Ok(((buf[1] as u16) << 8) + buf[0] as u16)
    } else {
        Ok(((buf[0] as u16) << 8) + buf[1] as u16)
    }
}

pub fn read_u32(file: &mut fs::File, endianness: Endianness) -> Result<u32, String> {
    let mut buf = [0; 4];
    let res = file.read(&mut buf);

    if res.is_err() {
        return Err(res.unwrap_err().to_string());
    } else if res.unwrap() < 4 {
        Err("reached end of file".to_string())
    } else if endianness == Endianness::LittleEndian {
        Ok(
            ((buf[3] as u32) << 24) + 
            ((buf[2] as u32) << 16) +
            ((buf[1] as u32) << 8) +
            buf[0] as u32
        )
    } else {
        Ok (
            ((buf[0] as u32) << 24) + 
            ((buf[1] as u32) << 16) +
            ((buf[2] as u32) << 8) +
            buf[3] as u32
        )
    }
}