//! The program's whole start (M9, 29 Oct 2026): the command line, where the game pack comes from, and the exit code.
//! The desktop binary (main.rs) and the Android library (`SDL_main` in the amigo_g5_android crate) both call `main`,
//! so a Windows exe and an APK start the same way the Linux binary does.
//!
//! The pack, in order: `--pack PACK.bin` or `--game DIR`; else the pack built into the binary (AMIGO_G5_PACK at build
//! time, as g5_package.sh builds the Windows exe); else `gamepack.bin` read through SDL_RWops, which on Android is the
//! APK asset (docs/G5-PLAYABLE.md, the packaging contract) and elsewhere a file in the working directory.

use std::fs;
use std::io::Read;
use std::path::PathBuf;

use amigo_g5::cpu::{Cpu, Stop};
use amigo_g5::game::GameTree;
use amigo_g5::mem::GuestMemory;
use amigo_g5::pack::Pack;

use crate::controls::Controls;
use crate::{parse_script, preboot, run, Options, Outcome};

// EMBEDDED_PACK: Option<&[u8]>, written by build.rs
include!(concat!(env!("OUT_DIR"), "/embedded.rs"));

/// The APK asset (or working-directory file) a binary with no pack argument and none built in reads.
pub const PACK_ASSET: &str = "gamepack.bin";

pub const USAGE: &str = "usage: amigo-g5-play (--pack PACK.bin | --game DIR [--config game.json]) [--program REL] [--fullscreen] [--ticks N --frames OUTDIR] [--script FILE] [--preboot-shot OUT.png [--preboot-at SECONDS]]";

/// The game's translated code by page, as in amigo-g5-boot (part = (PC >> 9) % 8).
pub fn game_step(cpu: &mut Cpu, mem: &mut GuestMemory) -> Result<(), Stop> {
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

/// Where the game comes from.
#[derive(Debug, PartialEq)]
pub enum Source {
    Pack(String),
    Dir(String),
    Embedded,
    Asset,
}

/// The value after flag `k`, if given.
fn get<'a>(v: &'a [String], k: &str) -> Option<&'a str> {
    v.iter()
        .position(|a| a == k)
        .and_then(|i| v.get(i + 1))
        .map(String::as_str)
}

/// The pack argument wins over a built-in pack, which wins over the asset.
pub fn source(args: &[String], embedded: bool) -> Source {
    match (get(args, "--pack"), get(args, "--game")) {
        (Some(p), _) => Source::Pack(p.into()),
        (None, Some(d)) => Source::Dir(d.into()),
        _ if embedded => Source::Embedded,
        _ => Source::Asset,
    }
}

/// `name` read whole through SDL_RWops: an APK asset on Android, a file elsewhere.
fn read_asset(name: &str) -> Result<Vec<u8>, String> {
    let mut rw = sdl2::rwops::RWops::from_file(name, "rb").map_err(|e| format!("{name}: {e}"))?;
    let mut b = Vec::new();
    rw.read_to_end(&mut b).map_err(|e| format!("{name}: {e}"))?;
    Ok(b)
}

fn options(v: &[String], embedded: Option<&[u8]>) -> Result<Options, String> {
    let read = |p: &str| fs::read(p).map_err(|e| format!("{p}: {e}"));
    let unpack = |b: &[u8], what: &str| {
        let pack = Pack::from_bytes(b).map_err(|e| format!("{what}: {e}"))?;
        Ok::<_, String>((pack.tree, pack.config))
    };
    let (tree, config) = match source(v, embedded.is_some()) {
        Source::Pack(p) => unpack(&read(&p)?, &p)?,
        Source::Dir(d) => (
            GameTree::from_dir(&PathBuf::from(&d)).map_err(|e| format!("{d}: {e}"))?,
            get(v, "--config").map(read).transpose()?,
        ),
        Source::Embedded => unpack(embedded.unwrap_or_default(), "built-in pack")?,
        Source::Asset => {
            eprintln!("play: pack {PACK_ASSET}");
            unpack(&read_asset(PACK_ASSET)?, PACK_ASSET)?
        }
    };
    let program = tree.program(get(v, "--program"))?;
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
        // Android shows every window full screen anyway; asking for it keeps the window the display's size
        fullscreen: cfg!(target_os = "android") || v.iter().any(|a| a == "--fullscreen"),
        ticks: get(v, "--ticks")
            .map(|t| t.parse().map_err(|_| "--ticks must be a number"))
            .transpose()?,
        frames: get(v, "--frames").map(PathBuf::from),
        script: match get(v, "--script") {
            Some(p) => parse_script(&String::from_utf8_lossy(&read(p)?))?,
            None => Vec::new(),
        },
    })
}

/// Runs the program with these arguments (the first is the program's name) and returns its exit code: 0 when the
/// player quits or N ticks ran, 3 when the game stopped early, 2 for bad arguments or files.
pub fn main<F>(v: &[String], embedded: Option<&[u8]>, step: &mut F) -> i32
where
    F: FnMut(&mut Cpu, &mut GuestMemory) -> Result<(), Stop>,
{
    let o = match options(v, embedded) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("amigo-g5-play: {e}\n{USAGE}");
            return 2;
        }
    };
    if let Some(out) = get(v, "--preboot-shot") {
        let at = get(v, "--preboot-at").map_or(Ok(0.0), |a| a.parse::<f32>());
        let r = match at {
            Ok(at) if (0.0..=30.0).contains(&at) => {
                preboot::shot(&o.title, &o.controls, at, out.as_ref())
            }
            _ => Err("--preboot-at must be seconds from 0 to 30".into()),
        };
        if let Err(e) = r {
            eprintln!("amigo-g5-play: {e}");
            return 2;
        }
        return 0;
    }
    if let Some(d) = &o.frames {
        if let Err(e) = fs::create_dir_all(d) {
            eprintln!("amigo-g5-play: {}: {e}", d.display());
            return 2;
        }
    }
    eprintln!("play: game code fnv {}", env!("AMIGO_G5_GAME_FNV"));
    // the TV remote's and the phone's Back arrive as the AC Back key instead of closing the activity, so Back opens
    // the pause menu and never just kills the game (docs/G5-PLAYABLE.md); must be set before SDL starts
    sdl2::hint::set("SDL_ANDROID_TRAP_BACK_BUTTON", "1");
    match run(&o, step) {
        Ok(Outcome::Quit) | Ok(Outcome::Ticks(_)) => 0,
        Ok(Outcome::Stopped(t, why)) => {
            eprintln!("STOP after {t} ticks: {why}");
            3
        }
        Err(e) => {
            eprintln!("amigo-g5-play: {e}");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(String::from).collect()
    }

    #[test]
    fn the_pack_argument_wins_then_the_built_in_pack_then_the_asset() {
        let a = args("play --pack p.bin --game d");
        assert_eq!(source(&a, true), Source::Pack("p.bin".into()));
        assert_eq!(
            source(&args("play --game d"), true),
            Source::Dir("d".into())
        );
        assert_eq!(source(&args("play"), true), Source::Embedded);
        assert_eq!(source(&args("play --fullscreen"), false), Source::Asset);
        // a flag with no value is no source
        assert_eq!(source(&args("play --pack"), false), Source::Asset);
    }

    #[test]
    fn a_damaged_built_in_pack_is_a_bad_file() {
        let mut no_step = |_: &mut Cpu, _: &mut GuestMemory| Ok(());
        assert_eq!(main(&args("play"), Some(b"AG5PACK1"), &mut no_step), 2);
    }
}
