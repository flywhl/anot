use anot;
use anyhow::Ok;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_cli_execution(c: &mut Criterion) {
    c.bench_function("Benchmark results", |b| {
        b.iter(|| {
            let args: Vec<String> = black_box(
                [
                    "./target/release/anot".to_string(),
                    "./rust".to_string(),
                    "--tags".to_string(),
                    "hypothesis,note,todo".to_string(),
                ]
                .to_vec(),
            );
            anot::cli::run(args)?;
            Ok(())
        })
    });
}

criterion_group!(benches, benchmark_cli_execution);
criterion_main!(benches);
