use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_multibigwig_summary(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-multibigwig-summary");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let a = manifest.join("tests/golden/a.bw");
    let b = manifest.join("tests/golden/b.bw");
    let out = tempfile::NamedTempFile::new().unwrap();

    c.bench_function("rsomics-multibigwig-summary golden", |bench| {
        bench.iter(|| {
            let status = Command::new(black_box(bin))
                .args([
                    "--bigwigfiles",
                    a.to_str().unwrap(),
                    b.to_str().unwrap(),
                    "--out-raw-counts",
                    out.path().to_str().unwrap(),
                ])
                .status()
                .unwrap();
            assert!(status.success());
        });
    });
}

criterion_group!(benches, bench_multibigwig_summary);
criterion_main!(benches);
