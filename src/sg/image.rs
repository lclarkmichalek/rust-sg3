use std::io::{Read, Seek, SeekFrom};
use util::*;
use sg::error::{Result, Error};

#[derive(Debug)]
pub struct ImageRecord {
    pub offset: u32,
    pub length: u32,
    pub uncompressed_length: u32,
    pub invert_offset: i32,
    pub width: i16,
    pub height: i16,
    pub image_type: u16,
    // the first flag indicates external images
    pub flags: [u8; 4],
    pub bitmap_id: u8,
    pub alpha_offset: u32,
    pub alpha_length: u32,
}

impl ImageRecord {
    pub fn read<T: Read + Seek>(r: &mut T, include_alpha: bool) -> Result<ImageRecord> {
        let offset = read_u32(r)?;
        let length = read_u32(r)?;
        let uncompressed_length = read_u32(r)?;
        r.seek(SeekFrom::Current(4))?;
        let invert_offset = read_i32(r)?;
        let width = read_i16(r)?;
        let height = read_i16(r)?;

        r.seek(SeekFrom::Current(26))?;
        let image_type = read_u16(r)?;

        let flags = [read_u8(r)?, read_u8(r)?, read_u8(r)?, read_u8(r)?];

        let bitmap_id = read_u8(r)?;
        r.seek(SeekFrom::Current(7))?;

        let alpha_offset = if include_alpha { read_u32(r)? } else { 0 };
        let alpha_length = if include_alpha { read_u32(r)? } else { 0 };

        if length < 0 {
            return Err(Error::MalformedFile(format!("invalid length {:?}", length)));
        }
        if width < 0 {
            return Err(Error::MalformedFile(format!("invalid width {:?}", width)));
        }
        if height < 0 {
            return Err(Error::MalformedFile(format!("invalid height {:?}", height)));
        }

        Ok(ImageRecord{
            offset: offset,
            length: length,
            uncompressed_length: uncompressed_length,
            invert_offset: invert_offset,
            width: width,
            height: height,
            image_type:image_type,
            flags: flags,
            bitmap_id: bitmap_id,
            alpha_offset: alpha_offset,
            alpha_length: alpha_length,
        })
    }

    pub fn load<T: Read + Seek>(&self, r: &mut T) -> Result<u8> {
        r.seek(SeekFrom::Start(self.offset as u64 - self.flags[0] as u64))?;
        let mut buf = vec![0, self.length];
        Ok(0)
    }
}
