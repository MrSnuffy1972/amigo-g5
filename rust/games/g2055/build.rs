//! Builds in the asset pack named by NATIVE_PACK (the binary form, napack), which is how the one-file desktop programs
//! are made. Without it the program reads `--pack` or the `gamepack.bin` asset (Android).
use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-env-changed=NATIVE_PACK");
    println!("cargo:rustc-check-cfg=cfg(native_embedded)");
    let mut src = "pub const EMBEDDED_PACK: Option<&[u8]> = None;\n".to_string();
    if let Some(p) = env::var("NATIVE_PACK").ok().filter(|p| !p.is_empty()) {
        let p = fs::canonicalize(&p).unwrap_or_else(|e| panic!("NATIVE_PACK {p}: {e}"));
        println!("cargo:rerun-if-changed={}", p.display());
        println!("cargo:rustc-cfg=native_embedded");
        src = format!(
            "pub const EMBEDDED_PACK: Option<&[u8]> = Some(include_bytes!({:?}));\n",
            p.display().to_string()
        );
    }
    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("embedded.rs"),
        src,
    )
    .unwrap();
}
