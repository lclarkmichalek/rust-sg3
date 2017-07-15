use std::io::{Read, Seek, SeekFrom};
use image;

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

    pub fn load<T: Read + Seek>(&self, r: &mut T) -> Result<TransparentImageDecoder> {
        r.seek(SeekFrom::Start(self.offset as u64 - self.flags[0] as u64))?;
        let mut input: Vec<u8> = vec![0; self.length as usize];
        r.read_exact(&mut input)?;
        let mut output: Vec<u8> = vec![0; 4 * (self.width as usize) * (self.height as usize)];
        read_transparent_image(&self, input.as_slice(), output.as_mut_slice())?;
        Ok(TransparentImageDecoder{
            rec: self,
            buf: output,
            curr_row: 0,
        })
    }
}

fn read_transparent_image(rec: &ImageRecord, input_buf: &[u8], output_buf: &mut [u8]) -> Result<usize> {
    if input_buf.len() != rec.length as usize {
        return Err(Error::MalformedFile(format!("buffer too short {:?} != {:?}", rec.length, input_buf.len())));
    }
    if output_buf.len() % 4 != 0 {
        return Err(Error::MalformedFile(format!("output was not multiple of 4: {:?}", output_buf.len())));
    }

    let mut skip: u8 = 0;
    let mut read: u8 = 0;
    let mut input = input_buf.iter();

    for output in output_buf.chunks_mut(4) {
        if skip == 0 && read == 0 {
            let c = match input.next() {
                None => return Err(Error::MalformedImage()),
                Some(b) => *b,
            };
            if c == 255 {
                skip = match input.next() {
                    None => return Err(Error::MalformedImage()),
                    Some(b) => *b,
                };
            } else {
                read = c;
            }
        }

        if read != 0 {
            let p = match input.next() {
                None => return Err(Error::MalformedImage()),
                Some(b) => *b,
            };
            let q = match input.next() {
                None => return Err(Error::MalformedImage()),
                Some(b) => *b,
            };
            let c: u32 = (p as u32) | (q as u32) << 8;
            let mut rgba: u32 = 0xff000000;
            rgba |= ((c & 0x7c00) << 9) | ((c & 0x7000) << 4);
            rgba |= ((c & 0x3e0) << 6 ) | ((c & 0x300)      );
            rgba |= ((c & 0x1f) << 3  ) | ((c & 0x1c) >> 2  );
            output[2] = (rgba & 0x000000ff)        as u8;
            output[1] = ((rgba & 0x0000ff00) >> 8 ) as u8;
            output[0] = ((rgba & 0x00ff0000) >> 16) as u8;
            output[3] = 255;

            read -= 1;
        } else if skip != 0 {
            skip -= 1;
        }
    }

    Ok(output_buf.len())
}

pub struct TransparentImageDecoder<'a> {
    rec: &'a ImageRecord,
    buf: Vec<u8>,
    curr_row: u16,
}

impl<'a> image::ImageDecoder for TransparentImageDecoder<'a> {
    fn dimensions(&mut self) -> image::ImageResult<(u32, u32)> {
        Ok((self.rec.width as u32, self.rec.height as u32))
    }

    fn colortype(&mut self) -> image::ImageResult<image::ColorType> {
        // We take our 5,5,5 and return 8,8,8,8
        Ok(image::ColorType::RGBA(8))
    }

    fn row_len(&mut self) -> image::ImageResult<usize> {
        Ok(4 * self.rec.width as usize)
    }

    fn read_scanline(&mut self, buf: &mut [u8]) -> image::ImageResult<u32> {
        let start = self.curr_row as usize * self.rec.width as usize * 4;
        let end = start + buf.len() as usize;
        buf.clone_from_slice(&self.buf[start .. end]);
        Ok(buf.len() as u32)
    }

    fn read_image(&mut self) -> image::ImageResult<image::DecodingResult> {
        Ok(image::DecodingResult::U8(self.buf.clone()))
    }
}
