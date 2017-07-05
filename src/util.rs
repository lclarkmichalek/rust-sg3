use std::io::Read;
use std::string::String;
use sg::error::{Result, Error};

use byteorder::{ByteOrder, LittleEndian};

pub fn read_u32(r: &mut Read) -> Result<u32> {
    let mut buf: [u8; 4] = [0; 4];
    r.read_exact(&mut buf)?;
    return Ok(LittleEndian::read_u32(&buf));
}

pub fn read_i32(r: &mut Read) -> Result<i32> {
    let mut buf: [u8; 4] = [0; 4];
    r.read_exact(&mut buf)?;
    return Ok(LittleEndian::read_i32(&buf));
}

pub fn read_i16(r: &mut Read) -> Result<i16> {
    let mut buf: [u8; 2] = [0; 2];
    r.read_exact(&mut buf)?;
    return Ok(LittleEndian::read_i16(&buf));
}

pub fn read_u16(r: &mut Read) -> Result<u16> {
    let mut buf: [u8; 2] = [0; 2];
    r.read_exact(&mut buf)?;
    return Ok(LittleEndian::read_u16(&buf));
}

pub fn read_u8(r: &mut Read) -> Result<u8> {
    let mut buf: [u8; 1] = [0; 1];
    r.read_exact(&mut buf)?;
    return Ok(buf[0]);
}

pub fn read_string(r: &mut Read, n: usize) -> Result<String> {
    let mut buf: Vec<u8> = Vec::with_capacity(n);
    for i in 0..n {
        buf.push(0)
    }
    r.read_exact(buf.as_mut_slice())?;

    let mut term = n;
    for i in 0..n {
        if buf[i] == 0 {
            term = i;
            break;
        }
    }
    buf.split_off(term);

    String::from_utf8(buf).map_err(|err| {
        Error::MalformedFile(format!("failed to parse string: {}", err))
    })
}
