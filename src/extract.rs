use crate::errors::AppError;
use crate::methods::lsb;
use std::{fs::OpenOptions, io::Write, path::PathBuf};

pub fn extract(
    image_path: &PathBuf,
    output_path: Option<&PathBuf>,
    method: Option<&String>,
    key: Option<&String>,
    verbose: bool,
) -> Result<(), AppError> {
    let method = match method {
        Some(method) => method,
        None => return Err(AppError::NotImplemented),
    };

    let res = match method.as_str() {
        "LSB" => lsb::extract(image_path, key, verbose),
        _ => return Err(AppError::UnsupportedMethod),
    };

    match res {
        Ok(data) => {
            match output_path {
                Some(path) => {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(path)?;
                    file.write_all(&data)?;
                }
                None => println!("{}", String::from_utf8_lossy(&data)),
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}
