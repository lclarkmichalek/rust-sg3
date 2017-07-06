use std::io::{Read, Seek, SeekFrom};
use util::*;
use sg::bitmap::{BitmapRecord, RECORD_SIZE};
use sg::image::ImageRecord;
use sg::error::{Result, Error};

const HEADER_SIZE: i64 = 680;

#[derive(Debug)]
pub struct SG3File {
    pub header: Header,
    pub bitmaps: Vec<BitmapRecord>,
    pub images: Vec<ImageRecord>,
}

impl SG3File {
    pub fn read<T: Read + Seek>(r: &mut T) -> Result<SG3File> {
        let h = Header::read(r)?;

        let mut bitmaps: Vec<BitmapRecord> = Vec::with_capacity(h.num_bitmap_records as usize);
        for i in 0..h.num_bitmap_records {
            let rec = BitmapRecord::read(r)?;
            bitmaps.push(rec);
        }
        let empty_bitmaps: i64 = h.max_bitmap_records() as i64 - h.num_bitmap_records as i64;
        r.seek(SeekFrom::Current(RECORD_SIZE * empty_bitmaps))?;

        // First image is a 'dummy'
        ImageRecord::read(r, h.supports_alpha())?;

        let mut images: Vec<ImageRecord> = Vec::with_capacity(h.num_image_records as usize);
        for i in 0..h.num_image_records {
            let rec = ImageRecord::read(r, h.supports_alpha())?;
            images.push(rec);
        }

        Ok(SG3File{
            header: h,
            bitmaps: bitmaps,
            images: images,
        })
    }
}

#[derive(Debug)]
pub struct Header {
    pub filesize: u32,
    pub version: u32,
    pub unknown1: u32,
    pub max_image_records: i32,
    pub num_image_records: i32,
    pub num_bitmap_records: i32,
    pub num_bitmap_records_without_system: i32,
    pub total_filesize: u32,
    pub filesize_555: u32,
    pub filesize_external: u32,
}

impl Header {
    pub fn read<T: Read + Seek>(r: &mut T) -> Result<Header> {
        let h = Header{
            filesize: read_u32(r)?,
            version: read_u32(r)?,
            unknown1: read_u32(r)?,
            max_image_records: read_i32(r)?,
            num_image_records: read_i32(r)?,
            num_bitmap_records: read_i32(r)?,
            num_bitmap_records_without_system: read_i32(r)?,
            total_filesize: read_u32(r)?,
            filesize_555: read_u32(r)?,
            filesize_external: read_u32(r)?,
        };
        r.seek(SeekFrom::Current(HEADER_SIZE - 10 * 4))?;
        Ok(h)
    }

    pub fn max_bitmap_records(&self) -> u64 {
        if self.version == 0xd3 {
            100
        } else {
            200
        }
    }

    fn supports_alpha(&self) -> bool {
        self.version >= 0xd6
    }
}

