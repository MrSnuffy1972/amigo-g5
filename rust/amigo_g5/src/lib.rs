//! Ami-Go G5: the Rust cold-boot port. The C translation is the oracle for everything here.
//!
//! M1 (28 Oct 2026): guest memory and the AmigaDOS hunk loader. No emulated hardware, no raw pointers.
//! M2 (28 Oct 2026): register state and flag helpers for generated Rust (`m68k2c.py --emit rust`).
//! M3 (28 Oct 2026): a write journal in guest memory; randomised trials of every instruction class against the C.

pub mod cpu;
pub mod hunk;
pub mod mem;
