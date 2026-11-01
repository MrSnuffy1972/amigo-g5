#![cfg_attr(all(windows, native_embedded), windows_subsystem = "windows")]
//! The desktop program of game 2035 (Linux, Windows): the asset pack built in (NATIVE_PACK) or `--pack`.
//! `--ticks N --out DIR [--script FILE] [--every K] [--seed N]` runs headless for verification.

fn main() {
    let v: Vec<String> = std::env::args().collect();
    let code = native_engine::platform::sdl::app_main(&v, g2035::EMBEDDED_PACK, &g2035::spec());
    std::process::exit(code);
}
