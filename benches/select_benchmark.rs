use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use zfx_sortition::sortition::select;

fn criterion_benchmark(c: &mut Criterion) {
    let mut outputs = Vec::new();
    for _ in 0..3 {
        let output = rand::thread_rng().gen::<[u8; 32]>();
        outputs.push(output);
    }

    for vrf_output in outputs {
        c.bench_function("Select 1million", |b| {
            b.iter(|| {
                select(1000000, 1000000000000, 2500.0, &vrf_output);
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
