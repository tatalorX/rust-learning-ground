use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Placeholder benchmark - will work after implementing LruCache
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lru_insert", |b| {
        // After implementing:
        // let mut cache = p166::LruCache::new(1000);
        // b.iter(|| {
        //     for i in 0..100 {
        //         cache.insert(black_box(i), i);
        //     }
        // });
        b.iter(|| {
            black_box(42);
        });
    });

    c.bench_function("lru_get", |b| {
        // After implementing:
        // let mut cache = p166::LruCache::new(1000);
        // for i in 0..1000 {
        //     cache.insert(i, i);
        // }
        // b.iter(|| {
        //     for i in 0..100 {
        //         cache.get(&black_box(i));
        //     }
        // });
        b.iter(|| {
            black_box(42);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
