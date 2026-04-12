use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use ptfs_api::Ptfs;

fn io_write_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("io_write");

    group.measurement_time(Duration::from_secs(5));

    group.bench_function("write_1kb_objects", |b| {
        b.iter(|| {
            let mut fs = Ptfs::new();
            fs.mkfs();
            fs.mount();

            for i in 0..1000 {
                let data = vec![0u8; 1024];
                fs.write_object(i, data);
            }
        })
    });

    group.finish();
}

fn io_read_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("io_read");

    group.measurement_time(Duration::from_secs(5));

    group.bench_function("read_1kb_objects", |b| {
        b.iter(|| {
            let mut fs = Ptfs::new();
            fs.mkfs();
            fs.mount();

            for i in 0..1000 {
                fs.write_object(i, vec![0u8; 1024]);
            }

            for i in 0..1000 {
                let _ = fs.read_object(&i);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, io_write_benchmark, io_read_benchmark);
criterion_main!(benches);
