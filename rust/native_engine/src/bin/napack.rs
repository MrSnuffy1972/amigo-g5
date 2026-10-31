//! napack: packs an asset-pack directory (manifest.json and its files) into the binary form a game embeds or an APK
//! carries, and lists a binary pack.
//!
//!   napack pack DIR OUT.nap      napack list PACK.nap

use native_engine::pack::Pack;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let r = match a.get(1).map(String::as_str) {
        Some("pack") if a.len() == 4 => Pack::from_dir(a[2].as_ref()).and_then(|p| {
            let b = p.to_bytes();
            std::fs::write(&a[3], &b).map_err(|e| e.to_string())?;
            println!(
                "{}: {} assets, {} bytes",
                a[3],
                p.manifest["assets"].as_array().map_or(0, Vec::len),
                b.len()
            );
            Ok(())
        }),
        Some("list") if a.len() == 3 => std::fs::read(&a[2])
            .map_err(|e| e.to_string())
            .and_then(|b| Pack::from_bytes(&b))
            .map(|p| {
                println!("{} ({})", p.title(), p.manifest["format"]);
                for x in p.manifest["assets"].as_array().into_iter().flatten() {
                    println!(
                        "  {:8} {:20} {} bytes",
                        x["kind"].as_str().unwrap_or(""),
                        x["name"].as_str().unwrap_or(""),
                        x["len"]
                    );
                }
            }),
        _ => Err("usage: napack pack DIR OUT.nap | napack list PACK.nap".into()),
    };
    if let Err(e) = r {
        eprintln!("napack: {e}");
        std::process::exit(2);
    }
}
