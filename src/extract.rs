use std::{io, path::PathBuf, process::exit};
use crate::methods::lsb;


pub fn extract(image_path: &PathBuf, output_path: Option<&PathBuf>, method:Option<&String>, key: Option<&String>, verbose: bool) -> io::Result<()>{
    let method = match method {
        Some(method) => method,
        None => {
            eprintln!("As of now, method detection is not supported");
            exit(-2);
        }
    };

    match method.as_str() {
        "LSB" => lsb::extract(image_path, output_path, key, verbose),
        _ => {
            eprintln!("Unsupported method: {}", method);
            exit(-1);
        }
    }
}
