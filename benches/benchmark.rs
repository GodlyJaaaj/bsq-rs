use bsq_rs::{file_to_string, get_biggest_square};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_find_largest_square(c: &mut Criterion) {
    let filename = "example_files/maps/intermediate_map_1000_1000_2".to_string();

    c.bench_function("find_largest_square intermediate_map_1000_1000_2", |b| {
        b.iter(|| {
            let (mut content, rows, cols) = file_to_string(&filename).unwrap();
            get_biggest_square(&mut content, rows, cols)
        })
    });

    let filename = "example_files/maps/intermediate_map_1000_1000".to_string();

    c.bench_function("find_largest_square intermediate_map_1000_1000", |b| {
        b.iter(|| {
            let (mut content, rows, cols) = file_to_string(&filename).unwrap();
            get_biggest_square(&mut content, rows, cols)
        })
    });

    let filename = "10000_10000_self_generated".to_string();

    c.bench_function("find_largest_square 10000_10000_self_generated", |b| {
        b.iter(|| {
            let (mut content, rows, cols) = file_to_string(&filename).unwrap();
            get_biggest_square(&mut content, rows, cols)
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = benchmark_find_largest_square
);

criterion_main!(benches);
