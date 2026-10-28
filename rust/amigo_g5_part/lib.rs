//! One of the eight crates that hold a game's translated code, by page (G5 M6, 28 Oct 2026; see build.rs for why).
//! Generated code: one match arm per instruction; its warnings are not this crate's.
#![allow(warnings, clippy::all)]
use amigo_g5::cpu::{sx16, sx8, Cpu, Stop};
use amigo_g5::mem::GuestMemory;

include!(concat!(env!("OUT_DIR"), "/part.rs"));
