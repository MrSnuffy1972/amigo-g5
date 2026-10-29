//! amigo-g5-play: the playable G5 binary (M7, 29 Oct 2026). Plays a game from a game pack or a directory of its files:
//!
//!   amigo-g5-play (--pack PACK.bin | --game DIR [--config game.json]) [--program REL] [--fullscreen]
//!                 [--ticks N --frames OUTDIR] [--script FILE] [--preboot-shot OUT.png [--preboot-at SECONDS]]
//!
//! Every start opens on the pre-boot menu (M8), except a `--ticks` run. `--preboot-shot` draws the pre-boot as it
//! looks SECONDS (default 0) into its countdown, as a 1920x1080 PNG, and exits without booting: drawn on the CPU, so
//! it needs no display.
//!
//! `--ticks` runs N ticks unpaced and exits, with `--frames` writing them as amigo-g5-boot does, so Ryzen can check
//! the two binaries' frames match. `--script` feeds scripted input (format in lib.rs `parse_script`). The game's
//! translated code is compiled in exactly as for amigo-g5-boot (AMIGO_G5_GAME_RS, the crates amigo_g5_part0..7).
//!
//! Exit 0 when the player quits or N ticks ran; 3 when the game stopped early (cause on stderr); 2 for bad arguments
//! or files.

use std::fs;
use std::path::PathBuf;

use amigo_g5::game::GameTree;
use amigo_g5::pack::Pack;
use amigo_g5_play::controls::Controls;
use amigo_g5_play::preboot;
use amigo_g5_play::{parse_script, run, Options, Outcome};

/// The game's translated code by page, as in amigo-g5-boot (part = (PC >> 9) % 8).
mod game {
    use amigo_g5::cpu::{Cpu, Stop};
    use amigo_g5::mem::GuestMemory;

    pub fn step(cpu: &mut Cpu, mem: &mut GuestMemory) -> Result<(), Stop> {
        match (cpu.pc >> 9) % 8 {
            0 => amigo_g5_part0::step(cpu, mem),
            1 => amigo_g5_part1::step(cpu, mem),
            2 => amigo_g5_part2::step(cpu, mem),
            3 => amigo_g5_part3::step(cpu, mem),
            4 => amigo_g5_part4::step(cpu, mem),
            5 => amigo_g5_part5::step(cpu, mem),
            6 => amigo_g5_part6::step(cpu, mem),
            _ => amigo_g5_part7::step(cpu, mem),
        }
    }
}

fn options() -> Result<Options, String> {
    let v: Vec<String> = std::env::args().collect();
    let get = |k: &str| {
        v.iter()
            .position(|a| a == k)
            .and_then(|i| v.get(i + 1).cloned())
    };
    let read = |p: &str| fs::read(p).map_err(|e| format!("{p}: {e}"));
    let (tree, config) = match (get("--pack"), get("--game")) {
        (Some(p), _) => {
            let pack = Pack::from_bytes(&read(&p)?).map_err(|e| format!("{p}: {e}"))?;
            (pack.tree, pack.config)
        }
        (None, Some(d)) => (
            GameTree::from_dir(&PathBuf::from(&d)).map_err(|e| format!("{d}: {e}"))?,
            get("--config").map(|c| read(&c)).transpose()?,
        ),
        _ => return Err("give --pack PACK.bin or --game DIR".into()),
    };
    let program = tree.program(get("--program").as_deref())?;
    if let Some(w) = &program.warning {
        eprintln!("play: WARNING {w}");
    }
    let name = config
        .as_deref()
        .and_then(|c| serde_json::from_slice::<serde_json::Value>(c).ok())
        .and_then(|j| j.get("name")?.as_str().map(String::from));
    Ok(Options {
        controls: Controls::from_game_json(config.as_deref())?,
        title: name.unwrap_or_else(|| program.rel.clone()),
        tree,
        program,
        fullscreen: v.iter().any(|a| a == "--fullscreen"),
        ticks: get("--ticks")
            .map(|t| t.parse().map_err(|_| "--ticks must be a number"))
            .transpose()?,
        frames: get("--frames").map(PathBuf::from),
        script: match get("--script") {
            Some(p) => parse_script(&String::from_utf8_lossy(&read(&p)?))?,
            None => Vec::new(),
        },
    })
}

fn main() {
    let o = match options() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("amigo-g5-play: {e}\nusage: amigo-g5-play (--pack PACK.bin | --game DIR [--config game.json]) [--program REL] [--fullscreen] [--ticks N --frames OUTDIR] [--script FILE] [--preboot-shot OUT.png [--preboot-at SECONDS]]");
            std::process::exit(2);
        }
    };
    let v: Vec<String> = std::env::args().collect();
    let get = |k: &str| v.iter().position(|a| a == k).and_then(|i| v.get(i + 1));
    if let Some(out) = get("--preboot-shot") {
        let at = get("--preboot-at").map_or(Ok(0.0), |a| a.parse::<f32>());
        let r = match at {
            Ok(at) if (0.0..=30.0).contains(&at) => {
                preboot::shot(&o.title, &o.controls, at, out.as_ref())
            }
            _ => Err("--preboot-at must be seconds from 0 to 30".into()),
        };
        if let Err(e) = r {
            eprintln!("amigo-g5-play: {e}");
            std::process::exit(2);
        }
        return;
    }
    if let Some(d) = &o.frames {
        if let Err(e) = fs::create_dir_all(d) {
            eprintln!("amigo-g5-play: {}: {e}", d.display());
            std::process::exit(2);
        }
    }
    eprintln!("play: game code fnv {}", env!("AMIGO_G5_GAME_FNV"));
    match run(&o, &mut game::step) {
        Ok(Outcome::Quit) | Ok(Outcome::Ticks(_)) => {}
        Ok(Outcome::Stopped(t, why)) => {
            eprintln!("STOP after {t} ticks: {why}");
            std::process::exit(3);
        }
        Err(e) => {
            eprintln!("amigo-g5-play: {e}");
            std::process::exit(2);
        }
    }
}
