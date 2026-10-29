//! amigo-g5-boot: the G5 boot binary (M6, 28 Oct 2026). Cold-boots a filed game from its own files, with no snapshot
//! of a reference run, and runs it headless for N ticks, writing judge.py's frame and audio dumps. Command line (the
//! G5 contract, docs/G5-RYZEN-CONTRACT.md section 2):
//!
//!   amigo-g5-boot --game DIR --config game.json --ticks N --frames OUTDIR --audio OUT.raw
//!
//! DIR holds the game disk's files as a directory tree (s/startup-sequence, the program, its data). The program is
//! `--program REL` when given (a path under DIR: the kit's G0 choice, g0.json `exe_rel`), else the first command of
//! s/startup-sequence that names a hunk executable at the top of DIR. Its argument string is the rest of the
//! startup-sequence line whose command names it, as oshle.py `startup_args` does; when no line names it (the kit
//! then starts it from Workbench, which the port does not serve yet) it gets an empty CLI line and a warning. The game's translated code is compiled in: `build.rs` takes the file named by the
//! environment variable AMIGO_G5_GAME_RS at build time (the output of `m68k2c.py --emit rust` for that game), spread
//! over the crates amigo_g5_part0..7 by page; without it every part is a stub that stops at the first instruction, so
//! the crates build and test with no game.
//!
//! Exit 0 when all N ticks ran; 3 when the run stopped early (an untranslated address, an OS call the port does not
//! serve yet, a bus error), with the cause on stderr and the frames up to the stop written; 2 for bad arguments or
//! files.

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use amigo_g5::cpu::Stop;
use amigo_g5::display::dump_frame;
use amigo_g5::game::GameTree;
use amigo_g5::run::Machine;

/// The game's translated code, spread over eight crates by page so that rustc builds it in parallel and each crate
/// stays small (G5 M6): the part is the PC's page number (PC >> 9, the emitter's PAGE_BITS) modulo 8.
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

struct Args {
    game: PathBuf,
    program: Option<String>,
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
        program: get("--program"),
        ticks: need("--ticks")?
            .parse()
            .map_err(|_| "--ticks must be a number".to_string())?,
        frames: PathBuf::from(need("--frames")?),
        audio: PathBuf::from(need("--audio")?),
    })
}

fn run(a: &Args) -> Result<u32, String> {
    let tree = GameTree::from_dir(&a.game).map_err(|e| e.to_string())?;
    let prog = tree.program(a.program.as_deref())?;
    if let Some(w) = &prog.warning {
        eprintln!("boot: WARNING {w}");
    }
    eprintln!("boot: game code fnv {}", env!("AMIGO_G5_GAME_FNV"));
    eprintln!(
        "boot: {} ({} bytes), arguments {:?}",
        prog.rel,
        tree.get(&prog.rel).map_or(0, |d| d.len()),
        String::from_utf8_lossy(&prog.args)
    );
    let booted = tree.boot(&prog)?;
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
    let os_trace = std::env::var("AMIGO_G5_OSTRACE").ok();
    if os_trace.is_some() {
        m.os_trace = Some(Vec::new());
    }
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
    if let (Some(path), Some(t)) = (os_trace, m.os_trace.as_ref()) {
        let text: String = t
            .iter()
            .map(|(tick, base, lvo)| format!("TICK {tick} CALL base ${base:X} LVO -${:X}\n", -lvo))
            .collect();
        fs::write(path, text).map_err(|e| e.to_string())?;
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
