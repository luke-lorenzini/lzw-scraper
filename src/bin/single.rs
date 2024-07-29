use demo::{lzw::LZW, new_maps};

use std::{
    fs,
    sync::{Arc, Mutex},
    time::Instant,
};

fn main() {
    let (compression_map, decompression_map) = new_maps();
    let compression_map = Arc::new(Mutex::new(compression_map));
    let decompression_map = Arc::new(Mutex::new(decompression_map));

    println!("Waiting for data...");

    let message = fs::read("./input_text").expect("No file");
    let start = Instant::now();

    let lzw = LZW;
    let res = lzw.compress(&message, compression_map.clone());
    println!("{:?}", res);
    let res = lzw.decompress(&res, decompression_map.clone());
    println!("{:?}", res);

    let dur = start.elapsed();
    println!("{:?}", dur);
}
