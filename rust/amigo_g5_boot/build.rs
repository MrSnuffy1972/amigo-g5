//! Puts the game's translated code into the boot binary (M6, 28 Oct 2026). AMIGO_G5_GAME_RS names the output of
//! `m68k2c.py --emit rust` for one game; without it the binary gets a stub that stops at the first instruction, so
//! the crate builds and its tests run with no game anywhere near the repo. The generated file's inner `#![allow]`
//! line is dropped (main.rs puts the same allowances on the module that includes it).
use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-env-changed=AMIGO_G5_GAME_RS");
    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("game.rs");
    let text = match env::var("AMIGO_G5_GAME_RS") {
        Ok(p) if !p.is_empty() => {
            println!("cargo:rerun-if-changed={p}");
            let t = fs::read_to_string(&p).unwrap_or_else(|e| panic!("AMIGO_G5_GAME_RS {p}: {e}"));
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
    fs::write(out, text).unwrap();
}
