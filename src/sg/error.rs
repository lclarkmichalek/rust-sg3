use std::result;
use std::error;
use std::io;
use std::fmt;

use image;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ImageError(image::ImageError),
    MalformedFile(String),
    MalformedImage(),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::IoError(ref e) => e.fmt(f),
            &Error::ImageError(ref e) => e.fmt(f),
            &Error::MalformedFile(ref e) => write!(f, "File was malformed: {:?}", e),
            &Error::MalformedImage() => write!(f, "Image byte stream was malformed"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(..) => &"IO error",
            Error::ImageError(..) => &"Image error",
            Error::MalformedFile(..) => &"Malformed file",
            Error::MalformedImage() => &"Malformed image",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::ImageError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Error {
        Error::ImageError(err)
    }
}
