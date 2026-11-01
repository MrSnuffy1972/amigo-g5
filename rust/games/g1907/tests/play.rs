//! Play tests on the real asset pack (G1907_PACK = the pack directory or .nap; skipped without it): fire starts a
//! one-player game, the bout runs (the clock counts down, fighters move and score), and a bout ends.

use native_engine::game::Options;
use native_engine::headless::{run, Run};
use native_engine::pack::Pack;

fn pack() -> Option<Pack> {
    let p = std::env::var("G1907_PACK").ok()?;
    let p = std::path::Path::new(&p);
    Some(if p.is_dir() {
        Pack::from_dir(p).unwrap()
    } else {
        Pack::from_bytes(&std::fs::read(p).unwrap()).unwrap()
    })
}

fn states(dir: &std::path::Path) -> Vec<serde_json::Value> {
    std::fs::read_to_string(dir.join("state.jsonl"))
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

#[test]
fn fire_starts_a_game_and_a_bout_is_scored() {
    let Some(p) = pack() else {
        eprintln!("G1907_PACK not set: skipped");
        return;
    };
    let out = std::env::temp_dir().join(format!("g1907-play-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&out);
    let mut text = String::from("100 key down Right Ctrl\n106 key up Right Ctrl\n");
    // walk in and attack now and then
    for k in 0..40 {
        let t = 300 + 40 * k;
        let key = ["Right", "Up", "Down", "Left"][k % 4];
        text += &format!(
            "{t} key down {key}\n{t} key down Right Ctrl\n{} key up Right Ctrl\n{} key up {key}\n",
            t + 8,
            t + 10
        );
    }
    let script = native_engine::script::parse(&text).unwrap();
    let r = Run {
        ticks: 2400,
        out: Some(out.clone()),
        every: 100000,
        script,
    };
    run(&g1907::spec(), &p, &Options::default(), &r).unwrap();
    let s = states(&out);
    let at = |t: usize, k: &str| s[t][k].as_i64().unwrap();
    assert_eq!(at(50, "phase"), 8, "the title first");
    assert_eq!(at(200, "phase"), 1, "fire started a bout");
    assert_eq!(at(200, "human0"), 1);
    assert!(at(1200, "timer") < at(200, "timer"), "the clock runs");
    let moved = (200..1200).any(|t| at(t, "x0") != at(200, "x0"));
    assert!(moved, "the player's fighter moves");
    let scored = (200..2400).any(|t| (0..3).any(|i| at(t, &format!("points{i}")) > 0));
    assert!(scored, "someone scores");
    assert!((200..2400).any(|t| at(t, "phase") == 3), "the bout ends");
    let _ = std::fs::remove_dir_all(&out);
}
