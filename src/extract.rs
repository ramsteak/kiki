use std::{fs::OpenOptions, io::{self, Write}, path::PathBuf, process::exit};
use crate::methods::lsb;


pub fn extract(image_path: &PathBuf, output_path: Option<&PathBuf>, method:Option<&String>, key: Option<&String>, verbose: bool) -> io::Result<()>{
    let method = match method {
        Some(method) => method,
        None => {
            eprintln!("As of now, method detection is not supported");
            exit(-2);
        }
    };

    let res = match method.as_str() {
        "LSB" => lsb::extract(image_path, key, verbose),
        _ => {
            eprintln!("Unsupported method: {}", method);
            exit(-1);
        }
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
                },
                None => println!("{}", String::from_utf8_lossy(&data)),
            }
            Ok(())
        },
        Err(e) => Err(e)
    }
}
