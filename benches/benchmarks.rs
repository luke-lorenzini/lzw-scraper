use criterion::{black_box, criterion_group, criterion_main, Criterion};
use demo::{lzw::LZW, new_maps};

use std::sync::{Arc, Mutex};

pub fn criterion_benchmark(c: &mut Criterion) {
    let message = Vec::from("TOBEORNOTTOBEORTOBEORNOT");
    let (compression_map, _) = new_maps();
    let compression_map = Arc::new(Mutex::new(compression_map));

    let thing = LZW::default();

    c.bench_function("compress", |b| {
        b.iter(|| thing.compress(black_box(&message), compression_map.clone()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
