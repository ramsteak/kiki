use std::{collections::HashSet, path::PathBuf};
use rand::{rngs::StdRng, Rng, SeedableRng};
use crate::errors::AppError;

use crate::methods::data::{hash_key,package_data};

use super::data;


pub fn embed(image_path: &PathBuf, output_path: &PathBuf, secret_data: &[u8], key: Option<&String>, verbose: bool) -> Result<(), AppError> {
    let img = image::open(image_path)?;

    let mut img = img.to_rgba8();
    
    let (width,height) = img.dimensions();
    let imgsize = width*height;
    if verbose {println!("Image size: {}x{}", width, height);}

    let message_len = secret_data.len() as u32;

    if message_len * 8 > (imgsize * 3) {return Err(AppError::DataOverflow)};

    let data = package_data(secret_data);
    let mut secret_bits = data::BitIterator::new(&data);

    let mut get_next_trip = || -> Option<(u8,u8,u8)> {
        let r = secret_bits.next()?;
        let g = secret_bits.next().unwrap_or(0);
        let b = secret_bits.next().unwrap_or(0);
        Some((r,g,b))
    };

    let mut rng = StdRng::seed_from_u64(hash_key(key));
    let mut used_pixels = HashSet::new();

    for _ in 0..(data.len()*8).div_ceil(3){
        let mut index;
        loop {
            index = rng.gen_range(0..imgsize);
            if !used_pixels.contains(&index) {used_pixels.insert(index); break}
        }
        let pixel = &mut img[(index % width, index / width)];

        match get_next_trip() {
            Some((r,g,b)) => {
                pixel[0] = pixel[0] & !1 | r;
                pixel[1] = pixel[1] & !1 | g;
                pixel[2] = pixel[2] & !1 | b;
            },
            None => break,
        }
    }

    img.save(output_path)?;

    if verbose {println!("Image saved");}

    Ok(())
}


pub fn extract(image_path: &PathBuf, key: Option<&String>, verbose: bool) -> Result<Vec::<u8>, AppError> {
    let img = image::open(image_path)?;
    let img = img.to_rgba8();


    let mut rng = StdRng::seed_from_u64(hash_key(key));
    let mut used_pixels = HashSet::new();
    let mut read_bits = Vec::with_capacity(33);  // minimum capacity is u32 in bits, rounded up to the nearest multiple of 3

    let (width,height) = img.dimensions();
    let imgsize = width*height;
    
    for _ in 0..11{ // reads the u32 corresponding to the message size, plus one extra bit
        let mut index;
        loop {
            index = rng.gen_range(0..imgsize);
            if !used_pixels.contains(&index) {used_pixels.insert(index); break}
        }
        let pixel = img[(index % width, index / width)];
        read_bits.push(pixel[0] & 1);
        read_bits.push(pixel[1] & 1);
        read_bits.push(pixel[2] & 1);
    };

    // Uses the first 32 bits to determine the total message length, and leaves the remaining bits
    let message_len: u32 = read_bits.drain(0..32).rev().enumerate().map(|(i, b)| (b as u32) << i).sum();
    if verbose {println!("Detected message length: {}", message_len)};
    
    let mut secret = Vec::<u8>::with_capacity(message_len as usize);

    // Keeps reading bits and storing into read_bits, when there are enough for a byte one is created and pushed into secret
    for _ in 0..(message_len * 8 -1).div_ceil(3){
        let mut index;
        loop {
            index = rng.gen_range(0..imgsize);
            if !used_pixels.contains(&index) {used_pixels.insert(index); break}
        }
        let pixel = img[(index % width, index / width)];
        read_bits.push(pixel[0] & 1);
        read_bits.push(pixel[1] & 1);
        read_bits.push(pixel[2] & 1);

        if read_bits.len() >= 8 {
            secret.push(read_bits.drain(0..8).rev().enumerate().map(|(i,b)| b<<i).sum());
        }
    }
    
    if verbose {println!("Message length: {}", message_len)};

    Ok(secret)

}
