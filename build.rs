use std::{
    env,
    fs::read_dir,
    path::{Path, PathBuf},
    process::Command,
};

fn compile(benchmark: &Path) {
    let stem = benchmark.file_stem().unwrap().to_str().unwrap();
    let rustc = env::var("RUSTC_BOEHM").expect("RUSTC_BOEHM environment var not specified");
    if stem.starts_with("bench_rustc") {
        Command::new("cargo")
            .args(&["script", benchmark.to_str().unwrap(), "--build-only"])
            .env("RUSTC", rustc.as_str())
            .output()
            .expect(format!("Failed to compile benchmark: {}", stem).as_str());
    } else if stem.starts_with("bench_rboehm") {
        Command::new("cargo")
            .args(&["script", benchmark.to_str().unwrap(), "--build-only"])
            .output()
            .expect(format!("Failed to compile benchmark: {}", stem).as_str());
    } else {
        panic!("Unable to compile benchmark {} - unknown compiler specified");
    };

    let mut bin = PathBuf::from(env::var("CARGO_HOME").unwrap());
    bin.push("binary-cache");
    bin.push("release");
    bin.push(&stem);

    let mut target = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    target.push("target");
    #[cfg(debug_assertions)]
    target.push("debug");
    #[cfg(not(debug_assertions))]
    target.push("release");

    Command::new("mv")
        .args(&[bin, target])
        .spawn()
        .expect("Couldn't move bin");
}

fn main() {
    for bm in read_dir("benchmarks/").unwrap() {
        compile(bm.unwrap().path().as_path());
    }
}
