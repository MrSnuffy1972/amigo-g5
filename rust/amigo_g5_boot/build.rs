//! The boot binary's build script (M6, 28 Oct 2026). The game's translated code itself is built by the crates
//! amigo_g5_part0..7 from the same AMIGO_G5_GAME_RS; this script only records the FNV-1a 64 of that file, which the
//! binary prints at boot, so the Ryzen glue can prove a binary holds the code it translated for that game (one build
//! directory serves every game). Without the variable the hash is "none" and every part is a stub.
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-env-changed=AMIGO_G5_GAME_RS");
    let mut fnv = "none".to_string();
    if let Ok(p) = env::var("AMIGO_G5_GAME_RS") {
        if !p.is_empty() {
            println!("cargo:rerun-if-changed={p}");
            let t = fs::read(&p).unwrap_or_else(|e| panic!("AMIGO_G5_GAME_RS {p}: {e}"));
            let h = t.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| {
                (h ^ b as u64).wrapping_mul(0x0100_0000_01b3)
            });
            fnv = format!("{h:016x}");
        }
    }
    println!("cargo:rustc-env=AMIGO_G5_GAME_FNV={fnv}");
}
