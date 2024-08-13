use clap::{Arg, ArgAction, Command};
use embed::embed;
use errors::AppError;
use extract::extract;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::exit;

mod embed;
mod errors;
mod extract;
mod help_text;
mod methods;

fn get_secret(fd_secret: &String) -> Result<Vec<u8>, AppError> {
    let mut secret = Vec::<u8>::new();

    match fd_secret.as_str() {
        "-" => io::stdin().read_to_end(&mut secret)?,
        _ => File::open(PathBuf::from(fd_secret))?.read_to_end(&mut secret)?,
    };
    Ok(secret)
}

fn main() {
    let cmd = Command::new("kiki")
        .version("0.1.0")
        .about("Steganography tool to embed into and retrieve data from image files.")
        .author("Ramsteak")
        .subcommand(
            Command::new("embed")
                .arg(
                    Arg::new("image")
                        .required(true)
                        .index(1)
                        .help(help_text::EMBED_IMAGE),
                )
                .arg(
                    Arg::new("output")
                        .required(true)
                        .index(2)
                        .help(help_text::EMBED_OUTPUT),
                )
                .arg(Arg::new("secret").index(3).help(help_text::EMBED_SECRET))
                .arg(
                    Arg::new("method")
                        .short('m')
                        .long("method")
                        .help(help_text::METHOD),
                )
                .arg(Arg::new("key").short('k').long("key").help(help_text::KEY))
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .action(ArgAction::SetTrue)
                        .help(help_text::VERBOSE),
                )
                .arg(
                    Arg::new("options")
                        .short('o')
                        .long("options")
                        .num_args(1..)
                        .help(help_text::OPTIONS),
                ),
        )
        .subcommand(
            Command::new("extract")
                .arg(
                    Arg::new("image")
                        .required(true)
                        .index(1)
                        .help(help_text::EXTRACT_IMAGE),
                )
                .arg(Arg::new("output").index(2).help(help_text::EXTRACT_OUTPUT))
                .arg(
                    Arg::new("method")
                        .short('m')
                        .long("method")
                        .help(help_text::METHOD),
                )
                .arg(Arg::new("key").short('k').long("key").help(help_text::KEY))
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .action(ArgAction::SetTrue)
                        .help(help_text::VERBOSE),
                )
                .arg(
                    Arg::new("options")
                        .short('o')
                        .long("options")
                        .num_args(1..)
                        .help(help_text::OPTIONS),
                ),
        )
        .after_help(help_text::AFTER_HELP)
        .get_matches();

    match cmd.subcommand() {
        Some(("embed", sub)) => {
            let image = PathBuf::from(sub.get_one::<String>("image").unwrap());
            let output = PathBuf::from(sub.get_one::<String>("output").unwrap());

            let method = sub.get_one::<String>("method");
            let key = sub.get_one::<String>("key");

            let verbose = sub.get_flag("verbose");

            let fd_secret = sub.get_one::<String>("secret").unwrap();

            let secret = match get_secret(fd_secret) {
                Ok(secret) => secret,
                Err(err) => {
                    eprintln!("Error in reading secret file {}: {}", fd_secret, err);
                    exit(-1);
                }
            };

            let options = sub
                .get_many::<String>("options")
                .map(|v| v.collect::<Vec<_>>())
                .unwrap_or_default();

            if verbose {
                println!("Kiki embed");
                println!("Image:        {}", image.to_str().unwrap());
                println!("Output:       {}", output.to_str().unwrap());
                println!("Secret:       {} bytes", secret.len());
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

            if let Err(err) = embed(&image, &output, &secret, method, key, verbose, options) {
                eprintln!("{}", err);
                exit(-1);
            }
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
            if let Err(err) = extract(&image, output_ref, method, key, verbose, options) {
                eprintln!("{}", err);
                exit(-1);
            };
        }
        _ => {
            eprintln!("No subcommand used. Specify either 'embed' or 'extract'.");
            exit(-1);
        }
    }
}
