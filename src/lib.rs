use std::collections::HashMap;

pub mod lzw;

pub fn calculate_compression_ratio(uncompressed: u32, compressed: u32) -> f64 {
    1. - (compressed as f64 / uncompressed as f64)
}

pub fn new_maps() -> (HashMap<String, u32>, HashMap<u32, String>) {
    let mut compression_map = HashMap::new();
    let mut decompression_map = HashMap::new();

    (0..255).for_each(|i| {
        compression_map.insert((char::from(i)).to_string(), i as u32);
        decompression_map.insert(i as u32, (char::from(i)).to_string());
    });

    (compression_map, decompression_map)
}
