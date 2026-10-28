//! Puts the game's translated code into the boot binary (M6, 28 Oct 2026). AMIGO_G5_GAME_RS names the output of
//! `m68k2c.py --emit rust` for one game; without it the binary gets a stub that stops at the first instruction, so
//! the crate builds and its tests run with no game anywhere near the repo. The generated file's inner `#![allow]`
//! line is dropped (main.rs puts the same allowances on the module that includes it).
use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-env-changed=AMIGO_G5_GAME_RS");
    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("game.rs");
    let mut fnv = "none".to_string();
    let text = match env::var("AMIGO_G5_GAME_RS") {
        Ok(p) if !p.is_empty() => {
            println!("cargo:rerun-if-changed={p}");
            let t = fs::read_to_string(&p).unwrap_or_else(|e| panic!("AMIGO_G5_GAME_RS {p}: {e}"));
            // FNV-1a 64 of the file as generated, printed by the binary at boot, so the Ryzen glue can prove a
            // binary holds the code it translated for that game (one build directory serves every game)
            let h = t.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| {
                (h ^ b as u64).wrapping_mul(0x0100_0000_01b3)
            });
            fnv = format!("{h:016x}");
            t.lines()
                .filter(|l| !l.starts_with("#!["))
                .collect::<Vec<_>>()
                .join("\n")
        }
        _ => "use amigo_g5::cpu::{Cpu, Stop};\nuse amigo_g5::mem::GuestMemory;\n\
              pub fn step(cpu: &mut Cpu, _mem: &mut GuestMemory) -> Result<(), Stop> {\n\
              Err(Stop::Untranslated(cpu.pc))\n}\n"
            .to_string(),
    };
    println!("cargo:rustc-env=AMIGO_G5_GAME_FNV={fnv}");
    fs::write(out, text).unwrap();
}
