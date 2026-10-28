//! amigo-g5-boot: the G5 boot binary (M6, 28 Oct 2026). Cold-boots a filed game from its own files, with no snapshot
//! of a reference run, and runs it headless for N ticks, writing judge.py's frame and audio dumps. Command line (the
//! G5 contract, docs/G5-RYZEN-CONTRACT.md section 2):
//!
//!   amigo-g5-boot --game DIR --config game.json --ticks N --frames OUTDIR --audio OUT.raw
//!
//! DIR holds the game disk's files as a directory tree (s/startup-sequence, the program, its data). The program is
//! the first command of s/startup-sequence that names a hunk executable at the top of DIR; the rest of that line is
//! its argument string. The game's translated code is compiled in: `build.rs` takes the file named by the
//! environment variable AMIGO_G5_GAME_RS at build time (the output of `m68k2c.py --emit rust` for that game) and,
//! without it, a stub that stops at the first instruction, so the crate builds and tests with no game.
//!
//! Exit 0 when all N ticks ran; 3 when the run stopped early (an untranslated address, an OS call the port does not
//! serve yet, a bus error), with the cause on stderr and the frames up to the stop written; 2 for bad arguments or
//! files.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use amigo_g5::boot::cold_boot;
use amigo_g5::cpu::Stop;
use amigo_g5::display::dump_frame;
use amigo_g5::os::GameFiles;
use amigo_g5::run::Machine;

// generated code: one match arm per instruction; its warnings are not this crate's
#[allow(warnings, clippy::all)]
mod game {
    include!(concat!(env!("OUT_DIR"), "/game.rs"));
}

struct Args {
    game: PathBuf,
    ticks: u32,
    frames: PathBuf,
    audio: PathBuf,
}

fn parse_args() -> Result<Args, String> {
    let v: Vec<String> = std::env::args().collect();
    let get = |k: &str| -> Option<String> {
        v.iter()
            .position(|a| a == k)
            .and_then(|i| v.get(i + 1).cloned())
    };
    let need = |k: &str| get(k).ok_or(format!("missing {k}"));
    Ok(Args {
        game: PathBuf::from(need("--game")?),
        ticks: need("--ticks")?
            .parse()
            .map_err(|_| "--ticks must be a number".to_string())?,
        frames: PathBuf::from(need("--frames")?),
        audio: PathBuf::from(need("--audio")?),
    })
}

/// Every file under `dir`, by its file name (the OS services look a name up by its last path part).
fn collect(dir: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) -> std::io::Result<()> {
    let mut ents: Vec<_> = fs::read_dir(dir)?.collect::<Result<_, _>>()?;
    ents.sort_by_key(|e| e.file_name());
    for e in ents {
        let p = e.path();
        if p.is_dir() {
            collect(&p, out)?;
        } else {
            out.push((p.clone(), fs::read(&p)?));
        }
    }
    Ok(())
}

fn is_hunk_exe(d: &[u8]) -> bool {
    d.len() >= 4 && d[..4] == [0, 0, 3, 0xF3]
}

/// The program s/startup-sequence starts: (file name, argument string ending in a newline).
fn startup_program(dir: &Path) -> Result<(String, Vec<u8>), String> {
    let s = ["s", "S"]
        .iter()
        .flat_map(|d| {
            ["startup-sequence", "Startup-Sequence", "STARTUP-SEQUENCE"]
                .iter()
                .map(move |f| dir.join(d).join(f))
        })
        .find(|p| p.is_file())
        .ok_or("no s/startup-sequence in the game's files")?;
    let text = fs::read(&s).map_err(|e| e.to_string())?;
    let top: Vec<String> = fs::read_dir(dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    for line in text.split(|&b| b == b'\n') {
        let line = String::from_utf8_lossy(line);
        let line = line.split(';').next().unwrap_or("").trim();
        let mut parts = line.splitn(2, char::is_whitespace);
        let Some(cmd) = parts.next().filter(|c| !c.is_empty()) else {
            continue;
        };
        let name = cmd.rsplit([':', '/']).next().unwrap_or(cmd);
        if let Some(f) = top.iter().find(|f| f.eq_ignore_ascii_case(name)) {
            let data = fs::read(dir.join(f)).map_err(|e| e.to_string())?;
            if is_hunk_exe(&data) {
                let mut args = parts.next().unwrap_or("").trim().as_bytes().to_vec();
                args.push(b'\n');
                return Ok((f.clone(), args));
            }
        }
    }
    Err("s/startup-sequence names no hunk executable at the top of the game's files".into())
}

fn run(a: &Args) -> Result<u32, String> {
    let (prog, args) = startup_program(&a.game)?;
    let mut all = Vec::new();
    collect(&a.game, &mut all).map_err(|e| e.to_string())?;
    let mut files = GameFiles::default();
    let mut exe = None;
    for (p, d) in all {
        let name = p.file_name().unwrap().to_string_lossy().into_owned();
        if p.parent() == Some(a.game.as_path()) && name == prog {
            exe = Some(d.clone());
        }
        files.add(&name, d);
    }
    let exe = exe.ok_or("program file vanished")?;
    eprintln!(
        "boot: {prog} ({} bytes), arguments {:?}",
        exe.len(),
        String::from_utf8_lossy(&args)
    );
    let booted = cold_boot(&exe, &args, files).map_err(|e| format!("cold boot failed: {e:?}"))?;
    eprintln!(
        "boot: entry ${:06X}, A7 ${:06X}",
        booted.cpu.pc, booted.cpu.a[7]
    );
    if let Ok(path) = std::env::var("AMIGO_G5_BOOTDUMP") {
        // dev check: guest RAM straight after the cold boot (chip RAM from $0, then slow RAM from $C00000), so the
        // game glue can show the translator read the same code bytes the cold boot loaded
        let mut d = Vec::new();
        for (base, len) in [
            (0u32, amigo_g5::boot::CHIP_SIZE),
            (0xC0_0000, amigo_g5::boot::SLOW_SIZE),
        ] {
            d.extend_from_slice(
                booted
                    .mem
                    .read_bytes(base, len)
                    .map_err(|e| e.to_string())?,
            );
        }
        fs::write(&path, d).map_err(|e| e.to_string())?;
    }
    let mut m = Machine::new(booted);
    fs::create_dir_all(&a.frames).map_err(|e| e.to_string())?;
    let mut wav = fs::File::create(&a.audio).map_err(|e| e.to_string())?;
    let mut step = game::step;
    for t in 1..=a.ticks {
        let r = m.run_tick(&mut step);
        let (px, snd) = m.take_output();
        let mut b = Vec::with_capacity(snd.len() * 2);
        for s in snd {
            b.extend_from_slice(&s.to_le_bytes());
        }
        wav.write_all(&b).map_err(|e| e.to_string())?;
        if let Err(stop) = r {
            let why = match stop {
                Stop::OsCall { base, lvo } => {
                    format!(
                        "an OS call the port does not serve: base ${base:X} LVO -${:X}",
                        -lvo
                    )
                }
                Stop::Untranslated(pc) => format!("untranslated address ${pc:06X}"),
                Stop::Mem(e) => format!("bus error: {e}"),
            };
            eprintln!(
                "STOP at tick {t}: {why} (PC ${:06X}, SR ${:04X}, A7 ${:06X}, D0 ${:08X}, A0 ${:08X}, A6 ${:08X}); {:?}",
                m.cpu.pc,
                m.cpu.get_sr(),
                m.cpu.a[7],
                m.cpu.d[0],
                m.cpu.a[0],
                m.cpu.a[6],
                m.stats
            );
            return Ok(t - 1);
        }
        dump_frame(&a.frames, t, &px).map_err(|e| e.to_string())?;
    }
    eprintln!(
        "ran {} ticks: {:?}, unmapped accesses {}",
        a.ticks,
        m.stats,
        m.mem.unmapped_accesses()
    );
    Ok(a.ticks)
}

fn main() {
    let a = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("amigo-g5-boot: {e}\nusage: amigo-g5-boot --game DIR --config game.json --ticks N --frames OUTDIR --audio OUT.raw");
            std::process::exit(2);
        }
    };
    match run(&a) {
        Ok(n) if n == a.ticks => {}
        Ok(_) => std::process::exit(3),
        Err(e) => {
            eprintln!("amigo-g5-boot: {e}");
            std::process::exit(2);
        }
    }
}
