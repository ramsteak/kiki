use std::path::PathBuf;

// use crate::methods::{lsb,kiki,jpeg};
use crate::methods::lsb;
use crate::errors::{AppError, ExtensionError};



fn supported_methods(extension: &str) -> Result<Vec<&'static str>, AppError>{
    match extension {
        "jpg" | "jpeg" | "jfif" | "pjpeg" | "pjp" => Ok(vec!["JPG"]),
        "bmp" | "png" => Ok(vec!["LSB", "KIKI"]),
        _ => Err(AppError::Extension(ExtensionError::UnsupportedExtension))
    }
}

pub fn embed(image_path: &PathBuf, output_path: &PathBuf, secret_data: &[u8], method:Option<&String>, key: Option<&String>, verbose: bool) -> Result<(), AppError>{

    let method = match output_path.extension().and_then(|e| e.to_str()){
        Some(extension) => {
            if verbose {println!("Output file has extension {}", extension)}

            let supported = supported_methods(extension)?;
            if verbose {println!("{} supports {:?}", extension, supported)}

            match method {
                Some(method) => {
                    if supported.contains(&method.as_str()){
                        method
                    }
                    else {return Err(AppError::UnsupportedMethod)}
                },
                None => &supported[0].to_string(),
            }
        },
        None => return Err(AppError::Extension(ExtensionError::MissingExtension))
    };
    if verbose {println!("Determined method: {}", method)};

    match method.as_str() {
        "LSB" => lsb::embed(image_path, output_path, secret_data, key, verbose),
        _ => Err(AppError::UnsupportedMethod)
    }
}
