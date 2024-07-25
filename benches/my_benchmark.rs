use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use demo::{fibonacci, LZW};

fn new_maps() -> (HashMap<String, u32>, HashMap<u32, String>) {
    

    let mut compression_map = HashMap::new();
    let mut decompression_map = HashMap::new();

    (0..255).for_each(|i| {
        compression_map.insert((char::from(i)).to_string(), i as u32);
        decompression_map.insert(i as u32, (char::from(i)).to_string());
    });

    (compression_map,decompression_map)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));


    let message = Vec::from("TOBEORNOTTOBEORTOBEORNOT");
    let (mut comp, _) = new_maps();

    let mut thing = LZW::default();
    // let result = thing.compress(message, &mut comp);

    c.bench_function("compress", |b| b.iter(|| thing.compress(message.clone(), &mut comp)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
