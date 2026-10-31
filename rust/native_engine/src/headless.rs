//! The headless runner: runs a game for N ticks as fast as possible with scripted input and writes what it did, for
//! verification against the original and for tests. No display, no SDL. Outputs: `fNNNNN.rgba` frames (logical size,
//! 0xAARRGGBB little-endian, no header) every `every` ticks, `state.jsonl` (one line a tick: the game's named state),
//! `audio.raw` (16-bit stereo at 44.1 kHz).

use std::io::Write;
use std::path::PathBuf;

use crate::audio::Mixer;
use crate::game::{Ctx, Flow, Options, Spec};
use crate::gfx::Frame;
use crate::input::{Host, Mapper};
use crate::pack::Pack;
use crate::save::Save;

pub const OUT_RATE: u32 = 44_100;

#[derive(Debug, Clone, Default)]
pub struct Run {
    pub ticks: u32,
    pub out: Option<PathBuf>,
    pub every: u32,
    pub script: Vec<(u32, Host)>,
}

/// Runs and returns the number of ticks run (fewer when the game quit).
pub fn run(spec: &Spec, pack: &Pack, opts: &Options, r: &Run) -> Result<u32, String> {
    let info = &spec.info;
    let mut game = (spec.make)(pack, opts)?;
    let mut mapper = Mapper::new(info.bindings.clone());
    let mut mixer = Mixer::new(OUT_RATE, info.tick_hz, info.effect_voices);
    let mut save = Save::memory();
    let mut frame = Frame::new(info.width, info.height);
    let mut argb = Vec::new();
    let err = |e: std::io::Error| e.to_string();
    let (mut st, mut au) = match &r.out {
        Some(d) => {
            std::fs::create_dir_all(d).map_err(err)?;
            (
                Some(std::io::BufWriter::new(
                    std::fs::File::create(d.join("state.jsonl")).map_err(err)?,
                )),
                Some(std::io::BufWriter::new(
                    std::fs::File::create(d.join("audio.raw")).map_err(err)?,
                )),
            )
        }
        None => (None, None),
    };
    let per_tick = (OUT_RATE / info.tick_hz.max(1)) as usize;
    let mut snd = Vec::new();
    for tick in 1..=r.ticks {
        for (_, h) in r.script.iter().filter(|(t, _)| *t == tick) {
            mapper.handle(h);
        }
        let input = mapper.tick();
        let flow = game.tick(&mut Ctx {
            input: &input,
            audio: &mut mixer,
            save: &mut save,
            tick,
        });
        snd.clear();
        mixer.render(per_tick, &mut snd);
        if let Some(a) = au.as_mut() {
            for s in &snd {
                a.write_all(&s.to_le_bytes()).map_err(err)?;
            }
        }
        if let Some(s) = st.as_mut() {
            let mut line = format!("{{\"t\":{tick}");
            for (k, v) in game.state() {
                line += &format!(",\"{k}\":{v}");
            }
            writeln!(s, "{line}}}").map_err(err)?;
        }
        if let (Some(d), true) = (&r.out, r.every > 0 && tick % r.every == 0) {
            game.render(&mut frame);
            frame.to_argb(&mut argb);
            let b: Vec<u8> = argb.iter().flat_map(|p| p.to_le_bytes()).collect();
            std::fs::write(d.join(format!("f{tick:05}.rgba")), b).map_err(err)?;
        }
        if flow == Flow::Quit {
            return Ok(tick);
        }
    }
    Ok(r.ticks)
}
