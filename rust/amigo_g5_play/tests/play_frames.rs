//! amigo-g5-play against the headless loop amigo-g5-boot runs (M7, 29 Oct 2026), on a synthetic program under SDL's
//! dummy video and audio drivers: under scripted input the play loop's frames equal, tick for tick, the frames of
//! the headless loop given by hand the Amiga input that script means. Also: a pack written by tools/g5_pack.py reads
//! back in Rust as the directory it was made from, and the play binary boots it. No game data: every byte is written
//! here.

use std::path::{Path, PathBuf};
use std::process::Command;

use amigo_g5::cpu::{Cpu, Stop};
use amigo_g5::display::dump_frame;
use amigo_g5::game::GameTree;
use amigo_g5::hardware::Input;
use amigo_g5::mem::GuestMemory;
use amigo_g5::pack::Pack;
use amigo_g5::run::Machine;
use amigo_g5_play::controls::Controls;
use amigo_g5_play::touch::layout;
use amigo_g5_play::{parse_script, run, Options, Outcome};

/// A hunk file with one CODE hunk of NOPs (HUNK_HEADER, HUNK_CODE, HUNK_END), and its startup-sequence.
fn synthetic_game(dir: &Path) {
    let mut h: Vec<u32> = vec![0x3F3, 0, 1, 0, 0, 4, 0x3E9, 4];
    h.extend([0x4E71_4E71; 4]);
    h.push(0x3F2);
    let b: Vec<u8> = h.iter().flat_map(|w| w.to_be_bytes()).collect();
    std::fs::create_dir_all(dir.join("s")).unwrap();
    std::fs::write(dir.join("Game"), b).unwrap();
    std::fs::write(dir.join("s/startup-sequence"), "Game\n").unwrap();
    std::fs::write(dir.join("data.bin"), [1u8, 2, 3]).unwrap();
}

/// The synthetic program, as translated code would run it: it reads the joystick (JOY1DAT), the fire buttons
/// (CIA-A PRA), the mouse counters (JOY0DAT) and the keyboard (CIA-A ICR and SDR, with the handshake), and paints
/// the background (COLOR00) from what it read, so every input shows in the frame.
fn step(cpu: &mut Cpu, mem: &mut GuestMemory) -> Result<(), Stop> {
    cpu.cyc += 40;
    let joy = mem.read_u16(0xDFF00C)?;
    let pra = mem.read_u8(0xBFE001)?;
    let mouse = mem.read_u16(0xDFF00A)?;
    if mem.read_u8(0xBFED01)? & 0x08 != 0 {
        cpu.d[7] = mem.read_u8(0xBFEC01)? as u32;
        mem.write_u8(0xBFEE01, 0x40)?;
        mem.write_u8(0xBFEE01, 0x00)?;
    }
    let c =
        (joy & 0x303) ^ (!pra as u16 & 0xC0) << 4 ^ mouse.rotate_left(3) ^ (cpu.d[7] as u16) << 1;
    mem.write_u16(0xDFF180, c & 0xFFF)?;
    Ok(())
}

const SCRIPT: &str = "\
3 key down Right
5 pad down a
6 key up Right
7 mouse 5 -3
8 pad up a
9 key down Space
10 key up Space
11 button down left
";
const TICKS: u32 = 14;

/// What SCRIPT means to the game at tick `t`, written out by hand.
fn expected(t: u32) -> (Input, Vec<(u8, bool)>) {
    let i = Input {
        right: (3..6).contains(&t),
        fire: (5..8).contains(&t),
        mouse_joy0dat: if t >= 7 { 0xFD05 } else { 0 },
        lmb: t >= 11,
        ..Input::default()
    };
    let keys = match t {
        9 => vec![(0x40, true)],
        10 => vec![(0x40, false)],
        _ => vec![],
    };
    (i, keys)
}

fn tmp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("g5play-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

#[test]
fn play_frames_match_headless_under_scripted_input() {
    std::env::set_var("SDL_VIDEODRIVER", "dummy");
    std::env::set_var("SDL_AUDIODRIVER", "dummy");
    let root = tmp("frames");
    synthetic_game(&root.join("game"));
    let tree = GameTree::from_dir(&root.join("game")).unwrap();
    let program = tree.program(None).unwrap();

    // the headless loop, as amigo-g5-boot runs it, with the input set by hand
    let head = root.join("head");
    std::fs::create_dir_all(&head).unwrap();
    let mut m = Machine::new(tree.boot(&program).unwrap());
    for t in 1..=TICKS {
        let (input, keys) = expected(t);
        m.chips(|c| {
            c.input = input;
            for (rk, down) in keys {
                c.cias.push_key(rk, down);
            }
        });
        m.run_tick(&mut step).unwrap();
        dump_frame(&head, t, &m.take_output().0).unwrap();
    }

    let play = root.join("play");
    std::fs::create_dir_all(&play).unwrap();
    let o = Options {
        tree,
        program,
        controls: Controls::default(),
        title: "synthetic".into(),
        fullscreen: false,
        ticks: Some(TICKS),
        frames: Some(play.clone()),
        script: parse_script(SCRIPT).unwrap(),
        touch: None,
    };
    assert_eq!(run(&o, &mut step).unwrap(), Outcome::Ticks(TICKS));

    // the same script with the stick and fire given by fingers on the on-screen controls (M9 part 2): the window is
    // the dummy driver's 1056x816, and the controls are where touch::layout puts them there
    let lay = layout(1056.0, 816.0, false);
    let (sx, sy, sr) = lay.stick;
    let (bx, by, _) = lay.buttons[0];
    let at = |x: f32, y: f32| format!("{} {}", x / 1056.0, y / 816.0);
    let touch_script = SCRIPT
        .replace(
            "3 key down Right",
            &format!("3 touch down 7 {}", at(sx + sr * 0.7, sy + 3.0)),
        )
        .replace(
            "6 key up Right",
            &format!("6 touch up 7 {}", at(sx + sr * 0.7, sy + 3.0)),
        )
        .replace("5 pad down a", &format!("5 touch down 9 {}", at(bx, by)))
        .replace(
            "8 pad up a",
            &format!(
                "8 touch move 9 {}\n8 touch up 9 {}",
                at(bx + 4.0, by),
                at(bx + 4.0, by)
            ),
        );
    assert_ne!(touch_script, SCRIPT);
    let touched = root.join("touch");
    std::fs::create_dir_all(&touched).unwrap();
    let o = Options {
        frames: Some(touched.clone()),
        script: parse_script(&touch_script).unwrap(),
        touch: Some(true),
        ..o
    };
    assert_eq!(run(&o, &mut step).unwrap(), Outcome::Ticks(TICKS));

    let mut distinct = std::collections::HashSet::new();
    for t in 1..=TICKS {
        let f = format!("p{t:05}.rgba");
        let a = std::fs::read(head.join(&f)).unwrap();
        let b = std::fs::read(play.join(&f)).unwrap();
        assert!(a == b, "tick {t}: play frame differs from headless");
        let c = std::fs::read(touched.join(&f)).unwrap();
        assert!(
            a == c,
            "tick {t}: frame under touch input differs from headless"
        );
        distinct.insert(a);
    }
    // not vacuous: the joystick, fire, mouse, key and button each changed the picture
    assert!(
        distinct.len() >= 6,
        "only {} distinct frames",
        distinct.len()
    );
    let _ = std::fs::remove_dir_all(&root);

    // a pack made by the Python packer reads back as the directory, and the play binary boots it
    let root = tmp("pack");
    synthetic_game(&root.join("game"));
    std::fs::write(root.join("game.json"), r#"{"name":"Synthetic"}"#).unwrap();
    let have_python = Command::new("python3").arg("--version").output().is_ok();
    if !have_python {
        eprintln!("SKIP pack check: no python3");
        return;
    }
    let tool = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/g5_pack.py");
    let pack = root.join("p.bin");
    let st = Command::new("python3")
        .arg(&tool)
        .args(["pack", "--game"])
        .arg(root.join("game"))
        .arg("--config")
        .arg(root.join("game.json"))
        .arg("--out")
        .arg(&pack)
        .status()
        .unwrap();
    assert!(st.success());
    let p = Pack::from_bytes(&std::fs::read(&pack).unwrap()).unwrap();
    assert_eq!(p.tree, GameTree::from_dir(&root.join("game")).unwrap());
    assert_eq!(p.config.as_deref(), Some(&br#"{"name":"Synthetic"}"#[..]));
    assert_eq!(Pack::from_bytes(&p.to_bytes()).unwrap(), p);
    // with no game code compiled in, every page is a stub: the binary boots the pack and stops at the first
    // instruction, as amigo-g5-boot does
    let out = Command::new(env!("CARGO_BIN_EXE_amigo-g5-play"))
        .arg("--pack")
        .arg(&pack)
        .args(["--ticks", "3", "--frames"])
        .arg(root.join("f"))
        .output()
        .unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(3), "{err}");
    assert!(err.contains("STOP after 0 ticks: Untranslated"), "{err}");
    let _ = std::fs::remove_dir_all(&root);
}
