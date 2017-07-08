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
        let mut output: Vec<u8> = vec![0; (4 * self.width * self.height) as usize];
        read_transparent_image(&self, input.as_slice(), output.as_mut_slice())?;
        Ok(TransparentImageDecoder{
            rec: self,
            buf: output,
            curr_row: 0,
        })
    }
}

fn read_transparent_image(rec: &ImageRecord, input: &[u8], output: &mut [u8]) -> Result<usize> {
    if input.len() != rec.length as usize {
        return Err(Error::MalformedFile(format!("buffer too short {:?} != {:?}", rec.length, input.len())));
    }
    if output.len() % 4 != 0 {
        return Err(Error::MalformedFile(format!("output was not multiple of 4: {:?}", input.len())));
    }

    let mut skip: u8 = 0;
    let mut i = 0;

    for output in output.chunks_mut(4) {
        if skip != 0 {
            skip -= 1;
            continue;
        }

        if i < input.len() {
            return Err(Error::MalformedFile(String::from("ran out of file")));
        }

        // if the 'control' byte is 255, the next byte is the number of bytes to skip. If not, it's
        // the number of bytes to read as 555 pixels
        let c = input[i];

        if c == 255 {
            i += 1;
            skip = input[i+1];
            continue
        }
        output[0] = ((input[i] | 0b00011111) << 3);
        output[1] = ((input[i] | 0b11100000) >> 2) | ((input[i+1] | 0b00000011) << 6);
        output[2] =                                  ((input[i+1] | 0b01111100) << 1);
        output[3] = 0;

        i += 2;
    }
    Ok(output.len())
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
