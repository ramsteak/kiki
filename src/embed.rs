use std::{io, path::PathBuf, process::exit};

// use crate::methods::{lsb,kiki,jpeg};
use crate::methods::lsb;


fn supported_methods(extension: &str) -> Vec<&'static str>{
    match extension {
        "jpg" | "jpeg" | "jfif" | "pjpeg" | "pjp" => vec!["JPG"],
        "bmp" | "png" => vec!["LSB", "KIKI"],
        
        _ => {
            eprintln!("Unsupported extension type");
            exit(-1)
        }
    }
}

pub fn embed(image_path: &PathBuf, output_path: &PathBuf, secret_data: &[u8], method:Option<&String>, key: Option<&String>, verbose: bool) -> io::Result<()>{

    let method = match output_path.extension().and_then(|e| e.to_str()){
        Some(extension) => {
            if verbose {println!("Output file has extension {}", extension)}

            let supported = supported_methods(extension);
            if verbose {println!("{} supports {:?}", extension, supported)}

            match method {
                Some(method) => {
                    if supported.contains(&method.as_str()){
                        method
                    }
                    else {
                        eprintln!("Output file does not support the required method. {} supports {:?}", extension, supported);
                        exit(-1);
                    }
                },
                None => &supported[0].to_string(),
            }
        },
        None => {
            eprintln!("Output file has no extension. Method must be specified");
            exit(-1);
        }
    };
    println!("Determined method: {}", method);

    match method.as_str() {
        "LSB" => lsb::embed(image_path, output_path, secret_data, key, verbose),
        _ => {
            eprintln!("Unsupported method: {}", method);
            exit(-1);
        }
    }
}
