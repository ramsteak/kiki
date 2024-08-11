use image;
use std::{fmt, io};

#[derive(Debug)]
pub enum ExtensionError {
    UnsupportedExtension,
    MissingExtension,
}

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Image(image::ImageError),
    Extension(ExtensionError),
    DataOverflow,
    UnsupportedMethod,
    NotImplemented,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<image::ImageError> for AppError {
    fn from(error: image::ImageError) -> Self {
        AppError::Image(error)
    }
}
