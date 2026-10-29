#![cfg_attr(all(windows, g5_embedded), windows_subsystem = "windows")]
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
//! The start itself is `amigo_g5_play::app::main`, shared with the Android library (M9). A product build (a pack
//! built in with AMIGO_G5_PACK, as tools/g5_package.sh makes it) is a Windows GUI program: no console window.
//!
//! Exit 0 when the player quits or N ticks ran; 3 when the game stopped early (cause on stderr); 2 for bad arguments
//! or files.

fn main() {
    let v: Vec<String> = std::env::args().collect();
    let code = amigo_g5_play::app::main(
        &v,
        amigo_g5_play::app::EMBEDDED_PACK,
        &mut amigo_g5_play::app::game_step,
    );
    std::process::exit(code);
}
