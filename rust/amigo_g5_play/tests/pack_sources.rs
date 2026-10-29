//! Where amigo-g5-play finds its game (M9, 29 Oct 2026): with no `--pack` or `--game` and no pack built in, it reads
//! `gamepack.bin` through SDL_RWops, the call that reads the APK asset on Android and a file in the working directory
//! here. No game data: the synthetic game is written here.

use std::process::Command;

use amigo_g5::game::GameTree;
use amigo_g5::pack::Pack;

fn play() -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_amigo-g5-play"));
    c.env("SDL_VIDEODRIVER", "dummy")
        .env("SDL_AUDIODRIVER", "dummy");
    c
}

#[test]
fn no_pack_argument_reads_the_gamepack_asset() {
    let root = std::env::temp_dir().join(format!("g5play-asset-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let game = root.join("game");
    std::fs::create_dir_all(game.join("s")).unwrap();
    // one CODE hunk of two NOPs: no code is compiled in, so the boot stops at the entry point
    let mut h: Vec<u32> = vec![0x3F3, 0, 1, 0, 0, 2, 0x3E9, 2];
    h.extend([0x4E71_4E71, 0x4E71_4E71, 0x3F2]);
    let b: Vec<u8> = h.iter().flat_map(|w| w.to_be_bytes()).collect();
    std::fs::write(game.join("Game"), b).unwrap();
    std::fs::write(game.join("s/startup-sequence"), "Game\n").unwrap();
    let pack = Pack {
        config: Some(br#"{"name": "Synth"}"#.to_vec()),
        tree: GameTree::from_dir(&game).unwrap(),
    };
    let run = root.join("run");
    std::fs::create_dir_all(&run).unwrap();
    let in_run = || play().current_dir(&run).args(["--ticks", "1"]).output();

    // nothing to read: a bad file, exit 2
    let o = in_run().unwrap();
    let err = String::from_utf8_lossy(&o.stderr);
    assert_eq!(o.status.code(), Some(2), "{err}");
    assert!(err.contains("gamepack.bin"), "{err}");

    // the asset boots as the same pack given by --pack does
    std::fs::write(run.join("gamepack.bin"), pack.to_bytes()).unwrap();
    let o = in_run().unwrap();
    let err = String::from_utf8_lossy(&o.stderr);
    assert_eq!(o.status.code(), Some(3), "{err}");
    assert!(err.contains("play: pack gamepack.bin"), "{err}");
    let by_arg = play()
        .args(["--ticks", "1", "--pack"])
        .arg(run.join("gamepack.bin"))
        .output()
        .unwrap();
    let tail = |e: &[u8]| String::from_utf8_lossy(e).lines().last().map(String::from);
    assert_eq!(tail(&o.stderr), tail(&by_arg.stderr));

    // a damaged asset is refused
    let mut bad = pack.to_bytes();
    let n = bad.len();
    bad[n / 2] ^= 1;
    std::fs::write(run.join("gamepack.bin"), bad).unwrap();
    let o = in_run().unwrap();
    assert_eq!(o.status.code(), Some(2));
    let _ = std::fs::remove_dir_all(&root);
}
