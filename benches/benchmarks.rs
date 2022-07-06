use cdc::{Rabin64, RollingHash64};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn slide_benchmarks(c: &mut Criterion) {
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

fn create_benchmarks(c: &mut Criterion) {
    c.bench_function("new", |b| {
        b.iter(|| Rabin64::new(5));
    });

    c.bench_function("with_polynom", |b| {
        b.iter(|| Rabin64::new_with_polynom(5, &0x3847fe406c36e1));
    });
}

criterion_group!(benches, slide_benchmarks, create_benchmarks);
criterion_main!(benches);
