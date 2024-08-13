use crate::errors::{AppError, AppErrorKind};
use rand::{rngs::StdRng, SeedableRng};
use std::{iter::zip, path::PathBuf};

use crate::methods::data::FromBits;
use crate::methods::data::{hash_key, package_data, BatchIterator, BitIterator};

use super::pixel::{PixelIterator, RandomPixelIterator, SequentialPixelIterator};

pub fn embed(
    image_path: &PathBuf,
    output_path: &PathBuf,
    secret_data: &[u8],
    key: Option<&String>,
    verbose: bool,
    options: Vec<&String>,
) -> Result<(), AppError> {
    let mut img = image::open(image_path)?.to_rgba8();

    let (width, height) = img.dimensions();
    let imgsize = width * height;
    if verbose {
        println!("Image size: {}x{}", width, height);
    }

    let message_len = secret_data.len() as u32;

    if message_len * 8 > (imgsize * 3) {
        return Err(AppError::new(
            crate::errors::AppErrorKind::DataOverflow,
            "Data is too long",
        ));
    };

    let data = package_data(secret_data);
    let secret_bits = BitIterator::new(&data);
    let bit_triplet = BatchIterator::new(secret_bits, 3);

    let iterpix = if options.contains(&&"SEQ".to_string()) {
        PixelIterator::Sequential(SequentialPixelIterator::new((width, height)))
    } else {
        let rng = StdRng::seed_from_u64(hash_key(key));
        PixelIterator::Random(RandomPixelIterator::new((width, height), rng))
    };
    // let iterpix = pixel::RandomPixelIterator::new((width, height), rng);

    for (trip, pix) in zip(bit_triplet, iterpix) {
        for (idx, val) in trip.iter().enumerate() {
            let pixel = &mut img[(pix.0, pix.1)];
            pixel[idx] = pixel[idx] & !1 | val.unwrap_or_default();
        }
    }

    img.save(output_path)?;

    if verbose {
        println!("Image saved");
    }

    Ok(())
}

pub fn extract(
    image_path: &PathBuf,
    key: Option<&String>,
    verbose: bool,
    options: Vec<&String>,
) -> Result<Vec<u8>, AppError> {
    let img = image::open(image_path)?.to_rgba8();

    let (width, height) = img.dimensions();
    if verbose {
        println!("Image size: {}x{}", width, height);
    }

    let mut iterpix = if options.contains(&&"SEQ".to_string()) {
        PixelIterator::Sequential(SequentialPixelIterator::new((width, height)))
    } else {
        let rng = StdRng::seed_from_u64(hash_key(key));
        PixelIterator::Random(RandomPixelIterator::new((width, height), rng))
    };

    let mut bitstream = (&mut iterpix).flat_map(|px| {
        let img = &img;
        (0..3).map(move |i| img[(px.0, px.1)][i] & 1)
    });

    let message_len =
        u32::from_bits((&mut bitstream).take(32).collect::<Vec<u8>>().as_slice()) as usize;
    if verbose {
        println!("Detected message length: {}", message_len)
    };

    if message_len >= 1048576 {
        println!(
            "The detected message length is {:.1} MB. Do you want to continue? (y/n)",
            (message_len as f32) / 1048576.0
        );
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim().to_lowercase();
        match line.chars().nth(0) {
            Some('y') => Ok(()),
            Some(_) | None => Err(AppError::new(
                AppErrorKind::UserStopped,
                "Operation stopped by user",
            )),
        }?;
    }

    let mut secret = Vec::<u8>::with_capacity(message_len);

    while secret.len() < message_len {
        let byte = u8::from_bits((&mut bitstream).take(8).collect::<Vec<u8>>().as_slice());
        secret.push(byte);
    }

    let crc_read = u32::from_bits((&mut bitstream).take(32).collect::<Vec<u8>>().as_slice());
    let crc_calc = crc32fast::hash(&secret);

    if verbose {
        println!("CRC32 in file    :    {}", crc_read);
        println!("Calculated CRC32 :    {}", crc_calc);
    }

    if crc_read == crc_calc {
        Ok(secret)
    } else {
        Err(AppError::new(
            AppErrorKind::CRCMismatch,
            "CRC32 mismatch: invalid data",
        ))
    }
}
