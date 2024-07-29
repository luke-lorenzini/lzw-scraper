use criterion::{black_box, criterion_group, criterion_main, Criterion};
use demo::{lzw::LZW, new_maps};

use std::sync::{Arc, Mutex};

pub fn criterion_benchmark(c: &mut Criterion) {
    let message_to_compress = Vec::from("TOBEORNOTTOBEORTOBEORNOT");
    let message_to_decompress = vec![
        84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263,
    ];

    let (compression_map, decompression_map) = new_maps();
    let compression_map = Arc::new(Mutex::new(compression_map));
    let decompression_map = Arc::new(Mutex::new(decompression_map));

    let thing = LZW::default();

    c.bench_function("compress", |b| {
        b.iter(|| thing.compress(black_box(&message_to_compress), compression_map.clone()))
    });

    c.bench_function("decompress", |b| {
        b.iter(|| thing.decompress(black_box(&message_to_decompress), decompression_map.clone()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
