use cdc::{Rabin64, RollingHash64};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box, Throughput};

pub fn slide_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("slide");
    let data = 16;
    for size in [1_000, 10_000, 100_000] {
        group.throughput(Throughput::Bytes(size));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter_batched(
                || Rabin64::new(5),
                |mut rabin| {
                    for _ in 0..size {
                        rabin.slide(black_box(&data));
                    }
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }
}

criterion_group!(benches, slide_benchmarks);
criterion_main!(benches);
