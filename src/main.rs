use std::{
    collections::HashMap,
    fs::read_dir,
    io::{self, Write},
    process::Command,
    time::Instant,
};

use rand::{seq::SliceRandom, thread_rng};
use statistical::{mean, standard_deviation};

const REPS_DEFAULT: usize = 2;
const ITERS: usize = 2;

#[inline(never)]
fn time(benchmark: &str) -> String {
    let mut bin = Command::new(format!("target/release/{}", benchmark));
    let before = Instant::now();
    for _ in 0..ITERS {
        bin.output().expect(&format!("Couldn't run {}", benchmark));
    }
    let d = Instant::now() - before;
    String::from(format!(
        "{:?}",
        d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9
    ))
}

fn mean_ci(d: &Vec<f64>) -> (f64, f64) {
    let m = mean(d);
    let sd = standard_deviation(d, None);
    // Calculate a 99% confidence based on the mean and standard deviation.
    (m, 2.58 * (sd / (d.len() as f64).sqrt()))
}

fn main() {
    let bmark_names = read_dir("benchmarks")
        .unwrap()
        .map(|x| {
            x.unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        })
        .collect::<Vec<_>>();

    let mut bmark_data: HashMap<_, Vec<f64>> = HashMap::new();
    for bn in &bmark_names {
        bmark_data.insert(bn, vec![]);
    }
    let mut rng = thread_rng();
    let mut done = 0;

    let reps = REPS_DEFAULT;

    while done < bmark_names.len() * reps {
        // Randomly select a benchmark to run next
        let bn = loop {
            let cnd = bmark_names.choose(&mut rng).unwrap();
            if bmark_data[cnd].len() < reps {
                break cnd;
            }
        };

        let stdout = time(&bn);
        let t = stdout.trim().parse::<f64>().unwrap();
        bmark_data.get_mut(&bn).unwrap().push(t);
        done += 1;
        print!(".");
        io::stdout().flush().ok();
    }
    println!();
    let mut bmark_names_sorted = bmark_names.clone();
    bmark_names_sorted.sort();
    for bn in &bmark_names_sorted {
        let (mean, ci) = mean_ci(&bmark_data[&bn]);
        println!("{}: {:.3} +/- {:.4}", bn, mean, ci);
    }
}
