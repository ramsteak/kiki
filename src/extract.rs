use crate::errors::{AppError, AppErrorKind};
use crate::methods::lsb;
use std::{fs::OpenOptions, io::Write, path::PathBuf};

pub fn extract(
    image_path: &PathBuf,
    output_path: Option<&PathBuf>,
    method: Option<&String>,
    key: Option<&String>,
    verbose: bool,
    options: Vec<&String>,
) -> Result<(), AppError> {
    let method = match method {
        Some(method) => method,
        None => {
            return Err(AppError::new(
                AppErrorKind::NotImplemented,
                "Method recognition is not yet implemented",
            ))
        }
    };

    let data = match method.as_str() {
        "LSB" => lsb::extract(image_path, key, verbose, options),
        method => Err(AppError::new(
            AppErrorKind::UnsupportedMethod,
            format!("{} is not a supported method.", method),
        )),
    }?;

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
