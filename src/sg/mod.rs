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
    use sg::image::ImageRecord;
    use sg::error::Result;

    fn read_sg3() -> Result<file::SG3File> {
        let mut f = File::open("/home/laurie/Downloads/SprAmbient.sg3")?;
        let file = file::SG3File::read(&mut f)?;
        Ok(file)
    }

    fn test_convert(output_filename: &str) -> Result<()> {
        let file = read_sg3()?;
        let mut i: u32 = 0;
        for img in file.images {
            i += 1;
            if img.length == 0 {
                println!("no length");
                continue;
            }
            if img.image_type != 256 {
                println!("not 256: {:?}", img.image_type);
                continue
            }

            if i != 4631 {
                continue;
            }
            println!("valid");

            let mut file555 = File::open("/home/laurie/Downloads/SprAmbient.555")?;
            let mut image_data = img.load_image_data(&mut file555)?;
            let mut filepng = File::create(output_filename)?;
            let mut enc = image::png::PNGEncoder::new(filepng);
            enc.encode(&image_data, img.width as u32, img.height as u32, image::ColorType::RGBA(8))?;
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

