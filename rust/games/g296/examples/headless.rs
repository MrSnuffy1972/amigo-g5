//! Headless verification runner without SDL (the same run as `g296 --ticks`):
//!   cargo run --release --example headless -- PACK TICKS OUTDIR [SCRIPT] [--every K] [--seed N]

use native_engine::game::Options;
use native_engine::headless::{run, Run};
use native_engine::pack::Pack;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let get = |k: &str| {
        a.iter()
            .position(|x| x == k)
            .and_then(|i| a.get(i + 1))
            .cloned()
    };
    let p = std::path::Path::new(&a[1]);
    let pack = if p.is_dir() {
        Pack::from_dir(p)
    } else {
        std::fs::read(p)
            .map_err(|e| e.to_string())
            .and_then(|b| Pack::from_bytes(&b))
    }
    .unwrap_or_else(|e| panic!("{e}"));
    let script = a
        .get(4)
        .filter(|s| !s.starts_with("--"))
        .map(|f| native_engine::script::parse(&std::fs::read_to_string(f).unwrap()).unwrap())
        .unwrap_or_default();
    let r = Run {
        ticks: a[2].parse().unwrap(),
        out: Some(a[3].clone().into()),
        every: get("--every").map(|s| s.parse().unwrap()).unwrap_or(1),
        script,
    };
    let o = Options {
        seed: get("--seed").map(|s| s.parse().unwrap()),
    };
    let n = run(&g296::spec(), &pack, &o, &r).unwrap_or_else(|e| panic!("{e}"));
    eprintln!("ran {n} ticks");
}
