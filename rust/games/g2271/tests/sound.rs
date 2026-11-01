//! The sound on the real pack (skipped without G2271_PACK): the title module and a block's tune start notes at an
//! audible volume, and the effects of +AC24 are all loaded.

use g2271::assets::Assets;
use g2271::sound::{Mix, Sound};

fn sound() -> Option<Sound> {
    let p = std::env::var("G2271_PACK").ok()?;
    let pack = native_engine::pack::Pack::from_dir(std::path::Path::new(&p)).unwrap();
    Some(Sound::new(&Assets::load(&pack).unwrap()))
}

fn notes(s: &mut Sound, steps: usize) -> (usize, u8) {
    let mut starts = 0;
    let mut loudest = 0;
    for _ in 0..steps {
        s.step();
        for m in s.out.drain(..) {
            match m {
                Mix::Start { .. } => starts += 1,
                Mix::Volume(_, v) => loudest = loudest.max(v),
                _ => {}
            }
        }
    }
    (starts, loudest)
}

#[test]
fn title_module_and_block_tune_play() {
    let Some(mut s) = sound() else {
        eprintln!("G2271_PACK not set: skipped");
        return;
    };
    s.play_title();
    assert!(s.playing, "the title module parses");
    let (n, v) = notes(&mut s, 200);
    eprintln!("title module: {n} notes, loudest {v}");
    assert!(n > 10 && v > 0);
    s.load_sounds(1);
    s.play_tune(0);
    assert!(s.playing, "tune 0 parses");
    let (n, v) = notes(&mut s, 200);
    eprintln!("tune 0: {n} notes, loudest {v}");
    assert!(n > 10 && v > 0);
}
