use image;
use std::{fmt, io};

#[derive(Debug)]
pub struct AppError {
    _kind: AppErrorKind,
    message: String,
}

#[derive(Debug)]
pub enum AppErrorKind {
    Io,
    Image,
    MissingExtension,
    UnsupportedExtension,
    DataOverflow,
    UnsupportedMethod,
    NotImplemented,
    CRCMismatch,
    UserStopped,
}

impl AppError {
    pub fn new(kind: AppErrorKind, message: impl Into<String>) -> Self {
        AppError {
            _kind: kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        let message = error.kind().to_string();
        AppError::new(AppErrorKind::Io, message)
    }
}

impl From<image::ImageError> for AppError {
    fn from(error: image::ImageError) -> Self {
        AppError::new(AppErrorKind::Image, format!("{}", error))
    }
}
