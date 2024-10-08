use std::path::PathBuf;

// use crate::methods::{lsb,kiki,jpeg};
use crate::errors::{AppError, AppErrorKind};
use crate::methods::lsb;

fn supported_methods(extension: &str) -> Result<Vec<&'static str>, AppError> {
    match extension {
        "bmp" | "png" => Ok(vec!["LSB"]),
        _ => Err(AppError::new(
            AppErrorKind::UnsupportedExtension,
            format!("{} is not yet supported.", extension),
        )),
    }
}

pub fn embed(
    image_path: &PathBuf,
    output_path: &PathBuf,
    secret_data: &[u8],
    method: Option<&String>,
    key: Option<&String>,
    verbose: bool,
    options: Vec<&String>,
) -> Result<(), AppError> {
    let method = match output_path.extension().and_then(|e| e.to_str()) {
        Some(extension) => {
            if verbose {
                println!("Output file has extension {}", extension)
            }

            let supported = supported_methods(extension)?;
            if verbose {
                println!("{} supports {:?}", extension, supported)
            }

            match method {
                Some(method) => {
                    if supported.contains(&method.as_str()) {
                        method
                    } else {
                        return Err(AppError::new(
                            AppErrorKind::UnsupportedMethod,
                            format!("{} is not a supported method.", method),
                        ));
                    }
                }
                None => &supported[0].to_string(),
            }
        }
        None => {
            return Err(AppError::new(
                AppErrorKind::MissingExtension,
                "Specified file is missing the extension.",
            ))
        }
    };
    if verbose {
        println!("Determined method: {}", method)
    };

    match method.as_str() {
        "LSB" => lsb::embed(image_path, output_path, secret_data, key, verbose, options),
        method => Err(AppError::new(
            AppErrorKind::UnsupportedMethod,
            format!("{} is not a supported method.", method),
        )),
    }
}
