//! Ami-Go G5: the Rust cold-boot port. The C translation is the oracle for everything here.
//!
//! M1 (28 Oct 2026): guest memory and the AmigaDOS hunk loader. No emulated hardware, no raw pointers.
//! M2 (28 Oct 2026): register state and flag helpers for generated Rust (`m68k2c.py --emit rust`).
//! M3 (28 Oct 2026): a write journal in guest memory; randomised trials of every instruction class against the C.
//! M4 (28 Oct 2026): cold boot of a filed program (its hunk file, files and entry point) and the exec/dos services it calls.
//! M5 (28 Oct 2026): the clean-sheet display (copper list as a table, frame composition, sprites, raw RGBA dumps).
//! M5 part 2 (28 Oct 2026): the clean-sheet blitter (each blit done at once by what it achieves, line draw included).
//! M5 part 3 (28 Oct 2026): Paula channels, the 44.1 kHz mix with its BLEP and RC filters, and the WAV dump.
//! M5 part 4 (28 Oct 2026): the CIAs (timers, time of day, interrupt control, fire buttons, keyboard link) and the
//! interrupt level.
//! M5 part 5 (28 Oct 2026): the custom chips as one runtime (register writes and reads, one PAL line at a call, the
//! progressive compose and the copper re-read on a mid-tick pointer write).
//! M6 (28 Oct 2026): the bus (custom chips and CIAs reached from guest memory) and the run loop that puts translated
//! code, the OS services and the chips on one clock; the boot binary `amigo-g5-boot` (crate amigo_g5_boot).

pub mod blit;
pub mod boot;
pub mod cia;
pub mod cpu;
pub mod display;
pub mod hardware;
pub mod hunk;
pub mod mem;
pub mod os;
pub mod paula;
pub mod run;
