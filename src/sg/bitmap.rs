use std::io::{Read, Seek, SeekFrom};
use util::*;
use sg::error::{Result};

pub const RECORD_SIZE: i64 = 200;

#[derive(Debug)]
pub struct BitmapRecord {
    pub filename: String,
    pub comment: String,
    pub width: u32,
    pub height: u32,
    pub num_images: u32,
    pub start_index: u32,
    pub end_index: u32,
}

pub fn read_bitmap_record<T: Read + Seek>(r: &mut T) -> Result<BitmapRecord> {
    let filename = read_string(r, 65)?;
    let comment = read_string(r, 51)?;

    let rec = BitmapRecord{
        filename: filename,
        comment: comment,
        width: read_u32(r)?,
        height: read_u32(r)?,
        num_images: read_u32(r)?,
        start_index: read_u32(r)?,
        end_index: read_u32(r)?,
    };

    r.seek(SeekFrom::Current(RECORD_SIZE - (65 + 51 + 4 * 5)))?;

    Ok(rec)
}
