use demo::LZW;

use std::collections::HashMap;
use std::fs;

fn main() {
    let message = fs::read("./input_text").expect("Unable to read file");

    // let (comp, decomp) = build_custom_maps();
    // let mut thing = LZW::new(comp, decomp);
    let mut lzw = LZW::default();

    // let uncompressed_size = message.len() as u32;
    let res = lzw.compress(message);
    println!("{:?}", res);
    // let compressed_size = res.len() as u32;
    // let x = thing.calculate_compression_ratio(uncompressed_size, compressed_size);
    // println!("{:?}", x);

    let res = lzw.decompress(res);
    println!("{:?}", res);
}

fn _build_custom_maps() -> (HashMap<String, u32>, HashMap<u32, String>) {
    let mut compression_map = HashMap::new();
    let mut decompression_map = HashMap::new();

    compression_map.insert("A".to_owned(), 1);
    compression_map.insert("B".to_owned(), 2);
    compression_map.insert("C".to_owned(), 3);
    compression_map.insert("D".to_owned(), 4);
    compression_map.insert("E".to_owned(), 5);
    compression_map.insert("F".to_owned(), 6);
    compression_map.insert("G".to_owned(), 7);
    compression_map.insert("H".to_owned(), 8);
    compression_map.insert("I".to_owned(), 9);
    compression_map.insert("J".to_owned(), 10);
    compression_map.insert("K".to_owned(), 11);
    compression_map.insert("L".to_owned(), 12);
    compression_map.insert("M".to_owned(), 13);
    compression_map.insert("N".to_owned(), 14);
    compression_map.insert("O".to_owned(), 15);
    compression_map.insert("P".to_owned(), 16);
    compression_map.insert("Q".to_owned(), 17);
    compression_map.insert("R".to_owned(), 18);
    compression_map.insert("S".to_owned(), 19);
    compression_map.insert("T".to_owned(), 20);
    compression_map.insert("U".to_owned(), 21);
    compression_map.insert("V".to_owned(), 22);
    compression_map.insert("W".to_owned(), 23);
    compression_map.insert("X".to_owned(), 24);
    compression_map.insert("Y".to_owned(), 25);
    compression_map.insert("Z".to_owned(), 26);

    decompression_map.insert(1, "A".to_owned());
    decompression_map.insert(2, "B".to_owned());
    decompression_map.insert(3, "C".to_owned());
    decompression_map.insert(4, "D".to_owned());
    decompression_map.insert(5, "E".to_owned());
    decompression_map.insert(6, "F".to_owned());
    decompression_map.insert(7, "G".to_owned());
    decompression_map.insert(8, "H".to_owned());
    decompression_map.insert(9, "I".to_owned());
    decompression_map.insert(10, "J".to_owned());
    decompression_map.insert(11, "K".to_owned());
    decompression_map.insert(12, "L".to_owned());
    decompression_map.insert(13, "M".to_owned());
    decompression_map.insert(14, "N".to_owned());
    decompression_map.insert(15, "O".to_owned());
    decompression_map.insert(16, "P".to_owned());
    decompression_map.insert(17, "Q".to_owned());
    decompression_map.insert(18, "R".to_owned());
    decompression_map.insert(19, "S".to_owned());
    decompression_map.insert(20, "T".to_owned());
    decompression_map.insert(21, "U".to_owned());
    decompression_map.insert(22, "V".to_owned());
    decompression_map.insert(23, "W".to_owned());
    decompression_map.insert(24, "X".to_owned());
    decompression_map.insert(25, "Y".to_owned());
    decompression_map.insert(26, "Z".to_owned());

    (compression_map, decompression_map)
}
