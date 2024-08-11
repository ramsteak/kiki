use clap::{Arg, ArgAction, Command};
use embed::embed;
use extract::extract;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::exit;

mod embed;
mod errors;
mod extract;
mod methods;

fn main() {
    let cmd = Command::new("kiki")
        .version("0.1.0")
        .about("Steganography tool to embed into and retrieve data from image files.")
        .author("Ramsteak")
        .subcommand(Command::new("embed")
            .arg(Arg::new("image").required(true).index(1)
                    .help("The path to the image to hide the data in."))
            .arg(Arg::new("output").required(true).index(2)
                    .help("Path of the output image."))
            .arg(Arg::new("secret").index(3)
                    .help("Path to the file containing the secret. If unspecified or \"-\", read from stdin."))
            .arg(Arg::new("method").short('m').long("method"))
            .arg(Arg::new("key").short('k').long("key"))
            .arg(Arg::new("verbose").short('v').long("verbose").action(ArgAction::SetTrue))
            .arg(Arg::new("options").short('o').long("options").num_args(1..))
        )
        .subcommand(Command::new("extract")
            .arg(Arg::new("image").required(true).index(1)
                .help("The path to the image to extract data from."))
            .arg(Arg::new("output").index(2)
                .help("The file path to write the data to. If unspecified or \"-\", write to stdout."))
            .arg(Arg::new("method").short('m').long("method"))
            .arg(Arg::new("key").short('k').long("key"))
            .arg(Arg::new("verbose").short('v').long("verbose").action(ArgAction::SetTrue))
            .arg(Arg::new("options").short('o').long("options").num_args(1..))
        )
        .after_help("\
Methods list:
    - LSB      Least significant bit. (lossless only)
    - JPG      Uses the jpg encoder to embed data. (jpg only)
    - KIK      Encodes data in 16x16 blocks distributed through the image, with dense LSB. (lossless only)
                Resists to cropping and rotation")
        .get_matches();

    match cmd.subcommand() {
        Some(("embed", sub)) => {
            let image = PathBuf::from(sub.get_one::<String>("image").unwrap());
            let output = PathBuf::from(sub.get_one::<String>("output").unwrap());

            let method = sub.get_one::<String>("method");
            let key = sub.get_one::<String>("key");

            let verbose = sub.get_flag("verbose");

            let fd_secret = sub.get_one::<String>("secret").unwrap();

            let mut secret = Vec::<u8>::new();
            let secret_size = match sub.get_one::<String>("secret").unwrap().as_str() {
                "-" => io::stdin()
                    .read_to_end(&mut secret)
                    .expect("Failed to read secret from stdin"),

                _ => File::open(PathBuf::from(fd_secret))
                    .expect("Failed to open secret file")
                    .read_to_end(&mut secret)
                    .expect("Failed to read secret from file"),
            };

            let options = sub
                .get_many::<String>("options")
                .map(|v| v.collect::<Vec<_>>())
                .unwrap_or_default();

            if verbose {
                println!("Kiki embed");
                println!("Image:        {}", image.to_str().unwrap());
                println!("Output:       {}", output.to_str().unwrap());
                println!("Secret:       {} bytes", secret_size);
                match method {
                    Some(method) => println!("Method:       {}", method),
                    None => println!("Method will be determined by filetype."),
                }
                match key {
                    Some(key) => println!("Key:          {}", key),
                    None => println!("Key not specified"),
                }
                println!("Options:      {:?}", options);
            }

            embed(&image, &output, &secret, method, key, verbose, options).expect("Error");
        }
        Some(("extract", sub)) => {
            let image = PathBuf::from(sub.get_one::<String>("image").unwrap());

            let output = sub
                .get_one::<String>("output")
                .filter(|&s| s != "-")
                .map(|s| PathBuf::from(s));

            let method = sub.get_one::<String>("method");
            let key = sub.get_one::<String>("key");

            let verbose = sub.get_flag("verbose");

            let options = sub
                .get_many::<String>("options")
                .map(|v| v.collect::<Vec<_>>())
                .unwrap_or_default();

            if verbose {
                println!("Kiki extract");
                println!("Image:        {}", image.to_str().unwrap());
                match &output {
                    Some(output) => println!("Output:       {}", output.to_str().unwrap()),
                    None => println!("Output to stdout"),
                }
                match method {
                    Some(method) => println!("Method:       {}", method),
                    None => println!("Method will be inferred"),
                }
                match key {
                    Some(key) => println!("Key:          {}", key),
                    None => println!("Key not specified"),
                }
                println!("Options:      {:?}", options);
            }
            let output_ref = output.as_ref();
            extract(&image, output_ref, method, key, verbose, options).expect("Error");
        }
        _ => {
            eprintln!("No subcommand used. Specify either 'embed' or 'extract'.");
            exit(-1);
        }
    }
}
