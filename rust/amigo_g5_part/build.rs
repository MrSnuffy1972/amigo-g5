//! One part of a game's translated code (G5 M6, 28 Oct 2026). rustc's front end grows much faster than linearly with
//! the size of one crate (measured on Ryzen: 100 pages of a large game checked in 5.2 s, 200 pages in 19 s, and the
//! whole 29 MB game had not finished after 10 minutes), so the code is spread over eight small crates that cargo
//! builds in parallel: this crate takes the pages whose number (PC >> PAGE_BITS) is `PART` modulo `PARTS`.
//!
//! Input: the file named by AMIGO_G5_GAME_RS (`m68k2c.py --emit rust`), which is a `step` dispatcher followed by one
//! `#[inline(never)] fn pXXX` per page. This script keeps the page functions of its part (made `pub`) and writes its
//! own `step` over them. Without the variable it writes a `step` that stops at once, so the crate builds with no game.
use std::{env, fs, path::Path};

const PARTS: u32 = 8;
const PAGE_BITS: u32 = 9;

fn main() {
    println!("cargo:rerun-if-env-changed=AMIGO_G5_GAME_RS");
    let name = env::var("CARGO_PKG_NAME").unwrap();
    let part: u32 = name
        .rsplit('_')
        .next()
        .and_then(|s| s.strip_prefix("part"))
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| panic!("package name {name} must end in _partN"));
    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("part.rs");
    let mut fns = String::new();
    let mut arms = String::new();
    if let Ok(p) = env::var("AMIGO_G5_GAME_RS") {
        if !p.is_empty() {
            println!("cargo:rerun-if-changed={p}");
            let t = fs::read_to_string(&p).unwrap_or_else(|e| panic!("AMIGO_G5_GAME_RS {p}: {e}"));
            // the boot binary picks the part as (PC >> 9) % 8, so the file must be paged the same way
            let want = format!("match cpu.pc >> {PAGE_BITS} {{");
            assert!(
                t.contains(&want),
                "AMIGO_G5_GAME_RS {p}: not paged by PC >> {PAGE_BITS} (m68k2rs.py PAGE_BITS must be {PAGE_BITS})"
            );
            for chunk in t.split("\n#[inline(never)]\nfn p").skip(1) {
                let hex: String = chunk
                    .chars()
                    .take_while(|c| c.is_ascii_hexdigit())
                    .collect();
                let page =
                    u32::from_str_radix(&hex, 16).unwrap_or_else(|_| panic!("page name p{hex}"));
                if page % PARTS != part {
                    continue;
                }
                // the chunk runs to the end of its function: the next page starts a new chunk, and the file ends after
                // the last one
                fns.push_str("#[inline(never)]\npub fn p");
                fns.push_str(chunk.trim_end());
                fns.push('\n');
                arms.push_str(&format!("        0x{hex} => p{hex}(cpu, mem),\n"));
            }
        }
    }
    let text = format!(
        "pub fn step(cpu: &mut Cpu, mem: &mut GuestMemory) -> Result<(), Stop> {{\n    match cpu.pc >> {PAGE_BITS} {{\n{arms}        _ => Err(Stop::Untranslated(cpu.pc)),\n    }}\n}}\n{fns}"
    );
    fs::write(out, text).unwrap();
}
