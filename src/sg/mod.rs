pub mod file;
pub mod image;
pub mod bitmap;
pub mod error;

#[cfg(test)]
mod tests {
    use std::io::*;
    use std::fs::*;

    use image;
    use image::ImageDecoder;

    use sg::file;
    use sg::image::{ImageRecord, TransparentImageDecoder};
    use sg::error::Result;

    fn read_sg3() -> Result<file::SG3File> {
        let mut f = File::open("/home/laurie/Downloads/SprAmbient.sg3")?;
        let file = file::SG3File::read(&mut f)?;
        Ok(file)
    }

    fn read_image(img: &ImageRecord) -> Result<TransparentImageDecoder> {
        let mut f = File::open("/home/laurie/Downloads/SprAmbient.555")?;
        let mut decoder = img.load(&mut f)?;
        Ok(decoder)
    }

    fn write_image(dec: &mut TransparentImageDecoder, f: &mut File) -> Result<()> {
        let mut enc = image::png::PNGEncoder::new(f);
        let (w, h) = dec.dimensions()?;
        let ct = dec.colortype()?;
        let buf: Vec<u8> = match dec.read_image()? {
            image::DecodingResult::U8(v) => v,
            _ => panic!("not a u8"),
        };
        enc.encode(&buf, w, h, ct)?;
        Ok(())
    }

    fn test_convert(output_filename: &str) -> Result<()> {
        let file = read_sg3()?;
        for img in file.images {
            if img.length == 0 {
                println!("no length");
                continue;
            }
            if img.image_type != 256 {
                println!("not 256: {:?}", img.image_type);
                continue
            }
            println!("valid");
            let mut dec = read_image(&img)?;
            let mut out_file = File::open(output_filename)?;
            write_image(&mut dec, &mut out_file)?;
            println!("written image");
            break;
        }
        Ok(())
    }

    #[test]
    fn test_read_header() {
        match test_convert("/tmp/output.png") {
            Err(why) => panic!("couldn't open sg3 file: {}", why),
            Ok(h) => h,
        };
        return;
    }
}

