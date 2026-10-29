//! `amigo-g5-play --preboot-shot` (M8, 29 Oct 2026): the binary draws the pre-boot of a synthetic game with no
//! display and writes a PNG that decodes to 1920x1080 with the countdown bar where the time says. No game data:
//! every byte is written here.

use std::process::Command;

/// The pixels of a PNG written with deflate's stored blocks (as theme::png writes it), checking every CRC.
fn decode(b: &[u8]) -> (u32, u32, Vec<u8>) {
    assert_eq!(&b[..8], b"\x89PNG\r\n\x1a\n");
    let (mut i, mut ihdr, mut z) = (8, Vec::new(), Vec::new());
    while i < b.len() {
        let n = u32::from_be_bytes(b[i..i + 4].try_into().unwrap()) as usize;
        let body = &b[i + 4..i + 8 + n];
        let crc = u32::from_be_bytes(b[i + 8 + n..i + 12 + n].try_into().unwrap());
        assert_eq!(amigo_g5_play::theme::crc32(body), crc);
        match &body[..4] {
            b"IHDR" => ihdr = body[4..].to_vec(),
            b"IDAT" => z.extend_from_slice(&body[4..]),
            _ => {}
        }
        i += 12 + n;
    }
    let (w, h) = (
        u32::from_be_bytes(ihdr[..4].try_into().unwrap()),
        u32::from_be_bytes(ihdr[4..8].try_into().unwrap()),
    );
    let (mut raw, mut j) = (Vec::new(), 2);
    loop {
        let last = z[j] & 1;
        let n = u16::from_le_bytes([z[j + 1], z[j + 2]]) as usize;
        raw.extend_from_slice(&z[j + 5..j + 5 + n]);
        j += 5 + n;
        if last == 1 {
            break;
        }
    }
    let adler = u32::from_be_bytes(z[j..j + 4].try_into().unwrap());
    assert_eq!(amigo_g5_play::theme::adler32(&raw), adler);
    (w, h, raw)
}

#[test]
fn preboot_shot_renders_without_a_display() {
    let d = std::env::temp_dir().join(format!("g5shot-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(d.join("game/s")).unwrap();
    std::fs::write(d.join("game/s/startup-sequence"), "Game\n").unwrap();
    let h: Vec<u32> = vec![0x3F3, 0, 1, 0, 0, 1, 0x3E9, 1, 0x4E71_4E71, 0x3F2];
    let b: Vec<u8> = h.iter().flat_map(|w| w.to_be_bytes()).collect();
    std::fs::write(d.join("game/Game"), b).unwrap();
    std::fs::write(
        d.join("game.json"),
        r#"{"name":"Synthetic","controls":{"labels":{"a":"Jump"},"label_status":{"a":"web"}}}"#,
    )
    .unwrap();
    let shot = |at: &str| {
        let out = d.join(format!("shot{at}.png"));
        let o = Command::new(env!("CARGO_BIN_EXE_amigo-g5-play"))
            .arg("--game")
            .arg(d.join("game"))
            .arg("--config")
            .arg(d.join("game.json"))
            .arg("--preboot-shot")
            .arg(&out)
            .args(["--preboot-at", at])
            .env("SDL_VIDEODRIVER", "dummy")
            .output()
            .unwrap();
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        decode(&std::fs::read(&out).unwrap())
    };
    let (w, h, raw) = shot("15");
    assert_eq!((w, h), (1920, 1080));
    assert_eq!(raw.len(), 1080 * (1920 * 3 + 1));
    // the countdown bar (design 560..1360 at y 906..924) is orange up to its middle at 15 s
    let px = |x: usize, y: usize| &raw[y * (1920 * 3 + 1) + 1 + x * 3..][..3];
    assert_eq!(px(700, 915), [0xFF, 0x88, 0x00]);
    assert_ne!(px(1200, 915), [0xFF, 0x88, 0x00]);
    let (_, _, raw0) = shot("0");
    assert_eq!(
        &raw0[915 * (1920 * 3 + 1) + 1 + 1200 * 3..][..3],
        [0xFF, 0x88, 0x00]
    );
    let bad = Command::new(env!("CARGO_BIN_EXE_amigo-g5-play"))
        .arg("--game")
        .arg(d.join("game"))
        .args(["--preboot-shot", "/dev/null", "--preboot-at", "31"])
        .output()
        .unwrap();
    assert_eq!(bad.status.code(), Some(2));
    let _ = std::fs::remove_dir_all(&d);
}
