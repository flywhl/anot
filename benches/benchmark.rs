use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::process::Command;

fn benchmark_cli_execution(c: &mut Criterion) {
    c.bench_function("Benchmark results", |b| {
        b.iter(|| {
            let output = Command::new("./target/release/anot")
                .arg("./rust/")
                .arg("--tags")
                .arg("hypothesis,note,todo")
                .output()
                .expect("Failed to execute");
            //println!("{:?}", output);
            black_box(output);
        })
    });
}

criterion_group!(benches, benchmark_cli_execution);
criterion_main!(benches);
