//! Ami-Go G5: the Rust cold-boot port. The C translation is the oracle for everything here.
//!
//! M1 (28 Oct 2026): guest memory and the AmigaDOS hunk loader. No emulated hardware, no raw pointers.
//! M2 (28 Oct 2026): register state and flag helpers for generated Rust (`m68k2c.py --emit rust`).

pub mod cpu;
pub mod hunk;
pub mod mem;
