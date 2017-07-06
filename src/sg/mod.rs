pub mod file;
pub mod image;
pub mod bitmap;
pub mod error;

#[cfg(test)]
mod tests {
    use std::io::*;
    use std::fs::*;

    use sg::file;
    use sg::error::Result;

    fn read_sg3() -> Result<file::SG3File> {
        let mut f = File::open("/home/laurie/Downloads/SprAmbient.sg3")?;
        let file = file::SG3File::read(&mut f)?;
        Ok(file)
    }

    #[test]
    fn test_read_header() {
        let file = match read_sg3() {
            Err(why) => panic!("couldn't open sg3 file: {}", why),
            Ok(h) => h,
        };
        println!("{:?}", file);
        for i in 0..file.images.len() {
            let img = &file.images[i];
            println!("{:?}", img.length);
        }
        println!("filesize: {:?}", file.header.filesize);
        println!("max #bmp: {:?}", file.header.max_bitmap_records());
        println!("#bmp: {:?}", file.header.num_bitmap_records);
        println!("#img: {:?}", file.header.num_image_records);
        println!("max #img: {:?}", file.header.max_image_records);
    }
}

