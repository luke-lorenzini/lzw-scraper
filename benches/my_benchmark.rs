use criterion::{black_box, criterion_group, criterion_main, Criterion};
use demo::{lzw::LZW, new_maps};

pub fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));

    let message = Vec::from("TOBEORNOTTOBEORTOBEORNOT");
    let (mut comp, _) = new_maps();

    let mut thing = LZW::default();
    // let result = thing.compress(message, &mut comp);

    c.bench_function("compress", |b| {
        b.iter(|| thing.compress(message.clone(), &mut comp))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
