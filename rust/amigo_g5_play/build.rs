//! The play binary's build script (M9, 29 Oct 2026). Records the FNV-1a 64 of AMIGO_G5_GAME_RS as
//! ../amigo_g5_boot/build.rs does (the same line, so Ryzen's glue proves which code a binary holds), and builds in the
//! game pack named by AMIGO_G5_PACK, which is how tools/g5_package.sh makes the one-file Windows exe. With a pack built
//! in, the cfg `g5_embedded` is set, which makes the Windows exe a GUI program (no console window). Without the
//! variable the binary has no pack and reads `--pack`, `--game` or the `gamepack.bin` asset.
use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-env-changed=AMIGO_G5_GAME_RS");
    println!("cargo:rerun-if-env-changed=AMIGO_G5_PACK");
    println!("cargo:rustc-check-cfg=cfg(g5_embedded)");
    let set = |k: &str| env::var(k).ok().filter(|p| !p.is_empty());
    let mut fnv = "none".to_string();
    if let Some(p) = set("AMIGO_G5_GAME_RS") {
        println!("cargo:rerun-if-changed={p}");
        let t = fs::read(&p).unwrap_or_else(|e| panic!("AMIGO_G5_GAME_RS {p}: {e}"));
        let h = t.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| {
            (h ^ b as u64).wrapping_mul(0x0100_0000_01b3)
        });
        fnv = format!("{h:016x}");
    }
    println!("cargo:rustc-env=AMIGO_G5_GAME_FNV={fnv}");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        // The bundled static SDL2 calls the registry API (RegOpenKeyExW, RegQueryValueExW, RegCloseKey); without
        // advapi32 the mingw link fails (measured on Ryzen 29 Oct 2026, the first G5 Windows package).
        println!("cargo:rustc-link-lib=advapi32");
    }
    let mut embedded = "pub const EMBEDDED_PACK: Option<&[u8]> = None;\n".to_string();
    if let Some(p) = set("AMIGO_G5_PACK") {
        let p = fs::canonicalize(&p).unwrap_or_else(|e| panic!("AMIGO_G5_PACK {p}: {e}"));
        println!("cargo:rerun-if-changed={}", p.display());
        println!("cargo:rustc-cfg=g5_embedded");
        embedded = format!(
            "pub const EMBEDDED_PACK: Option<&[u8]> = Some(include_bytes!({:?}));\n",
            p.display().to_string()
        );
    }
    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("embedded.rs");
    fs::write(out, embedded).unwrap();
}
