//! Differential test: the Rust hunk loader against `kit/amigo/hunkload.py` on synthetic hunk files built here.
//! No game data. Skips cleanly (with a message) when python3 is not available.

use amigo_g5::hunk::{load, HunkError, LoadOptions};
use std::path::PathBuf;
use std::process::Command;

const CHIP: u32 = 1 << 30;

// Runs hunkload.load() and prints BASE, ENTRY, RELOCS, ADDR list and the image as hex; ERR/UNSUP on failure.
const DRIVER: &str = r#"
import importlib.util, sys
spec = importlib.util.spec_from_file_location("hunkload", sys.argv[1])
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
base = None if sys.argv[3] == "auto" else int(sys.argv[3], 0)
try:
    r = m.load(open(sys.argv[2], "rb").read(), base, 8, 0x20000, sys.argv[4] == "1")
except NotImplementedError:
    print("UNSUP"); sys.exit(0)
except (ValueError, EOFError):
    print("ERR"); sys.exit(0)
print("BASE", r["base"]); print("ENTRY", r["entry"]); print("RELOCS", r["relocs"])
print("ADDR", " ".join(str(a) for a in r["addr"])); print("IMAGE", r["image"].hex())
"#;

fn long(v: &mut Vec<u8>, x: u32) {
    v.extend_from_slice(&x.to_be_bytes());
}

fn longs(v: &mut Vec<u8>, xs: &[u32]) {
    for &x in xs {
        long(v, x);
    }
}

fn header(sizes_longs: &[u32]) -> Vec<u8> {
    let mut v = Vec::new();
    longs(
        &mut v,
        &[
            0x3F3,
            0,
            sizes_longs.len() as u32,
            0,
            sizes_longs.len() as u32 - 1,
        ],
    );
    longs(&mut v, sizes_longs);
    v
}

fn body(v: &mut Vec<u8>, ty: u32, size_longs: u32, words: &[u32]) {
    longs(v, &[ty, size_longs]);
    longs(v, words);
}

/// Three hunks (code, data, bss) with RELOC32 both ways and an end-of-hunk marker after each.
fn three_hunks() -> Vec<u8> {
    let mut v = header(&[4, 3, 8]);
    body(
        &mut v,
        0x3E9,
        4,
        &[0x7001_7002, 0x4EB9_0000, 0x0010_4E75, 0x0000_0000],
    );
    longs(&mut v, &[0x3EC, 1, 1, 6, 0]); // reloc: 1 long at code+6 pointing into hunk 1
    longs(&mut v, &[0x3F0, 0x2, 0x41, 0x0, 0, 0]); // symbol (skipped)
    long(&mut v, 0x3F2);
    body(&mut v, 0x3EA, 3, &[0x0000_0000, 0x0000_0008, 0xAABB_CCDD]);
    longs(&mut v, &[0x3EC, 1, 0, 0, 1, 2, 4, 0]); // data+0 -> code; data+4 -> bss(2)? 2 targets below
    long(&mut v, 0x3F2);
    longs(&mut v, &[0x3EB, 8]);
    long(&mut v, 0x3F2);
    v
}

fn short_relocs() -> Vec<u8> {
    let mut v = header(&[2, 2]);
    body(&mut v, 0x3E9, 2, &[0x1111_2222, 0x3333_0004]);
    // RELOC32SHORT: 1 reloc, target hunk 1, offset 4; terminator; (count=1,target=1,off=4,0) = 4 words, no pad
    v.extend_from_slice(&0x3FCu32.to_be_bytes());
    for w in [1u16, 1, 4, 0] {
        v.extend_from_slice(&w.to_be_bytes());
    }
    long(&mut v, 0x3F2);
    body(&mut v, 0x3EA, 2, &[0x0000_0000, 0x0000_0010]);
    // RELRELOC32 with two offsets + terminator = 5 words, so it needs a 2-byte pad
    v.extend_from_slice(&0x3FDu32.to_be_bytes());
    for w in [2u16, 0, 0, 4, 0] {
        v.extend_from_slice(&w.to_be_bytes());
    }
    long(&mut v, 0x3F2);
    v
}

fn reloc16_8() -> Vec<u8> {
    let mut v = header(&[2, 1]);
    body(&mut v, 0x3E9, 2, &[0x0004_0000, 0x0000_0000]);
    longs(&mut v, &[0x3ED, 1, 1, 0, 0]); // 16-bit pc-relative into hunk 1, offset 0
    longs(&mut v, &[0x3EE, 1, 1, 7, 0]); // 8-bit at offset 7
    long(&mut v, 0x3F2);
    body(&mut v, 0x3EA, 1, &[0x5A5A_5A5A]);
    long(&mut v, 0x3F2);
    v
}

fn chip_and_fast() -> Vec<u8> {
    let mut v = header(&[2, 2 | CHIP, 3]);
    body(&mut v, 0x3E9, 2, &[0x7000_4E75, 0x0000_0000]);
    body(&mut v, 0x3EA | CHIP, 2, &[0x0000_0000, 0x0000_0000]);
    longs(&mut v, &[0x3EC, 1, 0, 4, 0]); // chip data+4 -> code
    long(&mut v, 0x3F2);
    longs(&mut v, &[0x3EB, 3]);
    longs(&mut v, &[0x3F1, 2, 0x44, 0x45]); // debug (skipped)
    long(&mut v, 0x3F2);
    v
}

fn all_cases() -> Vec<(&'static str, Vec<u8>)> {
    let mut named = header(&[1]);
    longs(&mut named, &[0x3E8, 1, 0x4142_4344]); // HUNK_NAME
    body(&mut named, 0x3E9, 1, &[0x4E75_4E71]);
    longs(&mut named, &[0x3F2, 0, 0]); // trailing zero padding
    let mut ext = header(&[1]);
    body(&mut ext, 0x3E9, 1, &[0x4E75_4E71]);
    longs(&mut ext, &[0x3EF, 0x0100_0001, 0x4142_4344, 0x10, 0, 0x3F2]);
    let mut ext_ref = header(&[1]);
    body(&mut ext_ref, 0x3E9, 1, &[0x4E75_4E71]);
    longs(&mut ext_ref, &[0x3EF, 0x8100_0001, 0x4142_4344, 0, 0x3F2]);
    let mut overlay = header(&[1]);
    body(&mut overlay, 0x3E9, 1, &[0x4E75_4E71]);
    longs(&mut overlay, &[0x3F5, 0, 0]);
    let mut extra = header(&[1]);
    body(&mut extra, 0x3E9, 1, &[0x4E75_4E71]);
    body(&mut extra, 0x3E9, 1, &[0x4E75_4E71]);
    let mut unknown = header(&[1]);
    longs(&mut unknown, &[0x3F8, 0, 0]);
    let mut reslib = vec![];
    longs(&mut reslib, &[0x3F3, 1, 0x4142_4344, 0, 0, 0, 0, 1]);
    body(&mut reslib, 0x3E9, 1, &[0x4E75_4E71]);
    let mut truncated = three_hunks();
    truncated.truncate(60);
    vec![
        ("single code hunk", {
            let mut v = header(&[1]);
            body(&mut v, 0x3E9, 1, &[0x7000_4E75]);
            long(&mut v, 0x3F2);
            v
        }),
        ("code data bss with reloc32", three_hunks()),
        ("reloc32short and relreloc32", short_relocs()),
        ("reloc16 and reloc8", reloc16_8()),
        ("chip and fast hunks", chip_and_fast()),
        ("name hunk and zero padding", named),
        ("ext definition", ext),
        ("ext reference (unsupported)", ext_ref),
        ("overlay (unsupported)", overlay),
        ("more hunks than declared", extra),
        ("unknown hunk type", unknown),
        ("resident library names", reslib),
        ("bad magic", vec![0, 0, 3, 0xE9, 0, 0, 0, 0]),
        ("truncated", truncated),
    ]
}

fn python_load(
    script: &std::path::Path,
    hunkload: &std::path::Path,
    file: &std::path::Path,
    base: Option<u32>,
    seglist: bool,
) -> Option<Vec<String>> {
    let out = Command::new("python3")
        .arg("-I")
        .arg(script)
        .arg(hunkload)
        .arg(file)
        .arg(base.map_or("auto".to_string(), |b| b.to_string()))
        .arg(if seglist { "1" } else { "0" })
        .output()
        .ok()?;
    assert!(
        out.status.success(),
        "python driver failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    Some(
        String::from_utf8(out.stdout)
            .unwrap()
            .lines()
            .map(str::to_owned)
            .collect(),
    )
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

#[test]
fn rust_loader_matches_hunkload_py() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let hunkload = root.join("kit/amigo/hunkload.py");
    if Command::new("python3").arg("--version").output().is_err() || !hunkload.exists() {
        eprintln!("SKIP: python3 or kit/amigo/hunkload.py not available");
        return;
    }
    let dir = std::env::temp_dir().join(format!("amigo_g5_hunk_diff_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let script = dir.join("driver.py");
    std::fs::write(&script, DRIVER).unwrap();

    let (mut compared, mut loaded) = (0, 0);
    for (name, bytes) in all_cases() {
        let file = dir.join("case.hunk");
        std::fs::write(&file, &bytes).unwrap();
        // default layout, an explicit non-chip base (spans the chip gap for chip_and_fast), and seglist headers
        for (base, seglist) in [
            (None, false),
            (Some(0xC0_0000), false),
            (None, true),
            (Some(0x40_0000), true),
        ] {
            let py = python_load(&script, &hunkload, &file, base, seglist).unwrap();
            let opts = LoadOptions {
                base,
                seglist,
                ..LoadOptions::default()
            };
            let rs = load(&bytes, &opts);
            let ctx = format!("{name} base={base:?} seglist={seglist}");
            match (py[0].split(' ').next().unwrap(), rs) {
                ("ERR", Err(HunkError::Invalid(_))) | ("UNSUP", Err(HunkError::Unsupported(_))) => {
                }
                ("BASE", Ok(l)) => {
                    loaded += 1;
                    assert_eq!(py[0], format!("BASE {}", l.base), "{ctx}");
                    assert_eq!(py[1], format!("ENTRY {}", l.entry), "{ctx}");
                    assert_eq!(py[2], format!("RELOCS {}", l.relocs), "{ctx}");
                    let addrs: Vec<String> = l.addr.iter().map(u32::to_string).collect();
                    assert_eq!(py[3], format!("ADDR {}", addrs.join(" ")), "{ctx}");
                    assert_eq!(py[4], format!("IMAGE {}", hex(&l.image)), "{ctx}");
                }
                (p, r) => panic!("{ctx}: python said {p}, rust said {:?}", r.map(|l| l.entry)),
            }
            compared += 1;
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(compared, all_cases().len() * 4);
    // 8 cases load cleanly in all four layouts: guard against a suite that only compares errors
    assert_eq!(loaded, 8 * 4);
}

#[test]
fn loaded_image_lands_in_guest_memory() {
    use amigo_g5::mem::GuestMemory;
    let bytes = three_hunks();
    let l = load(&bytes, &LoadOptions::default()).unwrap();
    let mut m = GuestMemory::a500(0x1000, 0x1000).unwrap();
    l.write_to(&mut m, false).unwrap();
    // reloc32 in hunk 0 at +6 pointed at hunk 1 (data); the original long there was 0x0010_4E75 spanning bytes 4..8
    let data = l.addr[1];
    assert_eq!(m.read_u32(data + 8), Ok(0xAABB_CCDD));
    assert_eq!(l.entry, l.addr[0]);
}
