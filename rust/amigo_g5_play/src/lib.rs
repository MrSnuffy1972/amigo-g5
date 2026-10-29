//! amigo-g5-play: the playable G5 runtime (M7, 29 Oct 2026, docs/G5-PLAYABLE.md). The same core as amigo-g5-boot
//! (the cold boot, `Machine::run_tick`, the frame and the 44.1 kHz mix); this crate adds an SDL2 window, a 50 Hz fixed
//! timestep decoupled from the display, an audio queue, the player's pad, keyboard and mouse mapped to what the game
//! reads, and the in-game pause menu. For the same game, inputs and tick the frames are the boot binary's (tested).

pub mod controls;
mod menu;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use amigo_g5::cpu::{Cpu, Stop};
use amigo_g5::display::{dump_frame, SCR_H, SCR_W};
use amigo_g5::game::{GameTree, Program};
use amigo_g5::mem::GuestMemory;
use amigo_g5::run::Machine;
use sdl2::controller::{Axis, Button};
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;

use controls::{Controls, Host, Mapper};
use menu::{Menu, MenuItem};

/// One tick: 50 Hz, the PAL frame rate the game was written for.
pub const TICK: Duration = Duration::from_millis(20);
/// Audio queued beyond this many ticks is dropped, so a slow host never lets the sound drift behind the picture.
const AUDIO_MAX_TICKS: u32 = 6;

pub struct Options {
    pub tree: GameTree,
    pub program: Program,
    pub controls: Controls,
    pub title: String,
    pub fullscreen: bool,
    /// Headless review and tests: run this many ticks as fast as possible (no pacing), then exit.
    pub ticks: Option<u32>,
    /// Write each tick's frame here in judge.py's format, as amigo-g5-boot's `--frames` does.
    pub frames: Option<PathBuf>,
    /// Scripted input: each event is taken just before its tick runs (ticks count from 1).
    pub script: Vec<(u32, Host)>,
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Quit,
    Ticks(u32),
    /// The game stopped early (an untranslated address, an OS call not served, a bus error) after this many ticks.
    Stopped(u32, String),
}

/// Scripted input, one event a line: `TICK key down|up NAME`, `TICK pad down|up NAME`, `TICK axis NAME X Y`,
/// `TICK mouse DX DY`, `TICK button down|up left|right`. Blank lines and `#` comments are skipped.
pub fn parse_script(text: &str) -> Result<Vec<(u32, Host)>, String> {
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let w: Vec<&str> = line
            .split('#')
            .next()
            .unwrap_or("")
            .split_whitespace()
            .collect();
        if w.is_empty() {
            continue;
        }
        let bad = || format!("script line {}: {line:?}", n + 1);
        let num = |s: &str| s.parse::<i32>().map_err(|_| bad());
        let tick: u32 = w[0].parse().map_err(|_| bad())?;
        let down = w.get(2).map(|s| *s == "down");
        let e = match (w.get(1).copied(), w.len()) {
            (Some("key"), 4..) => Host::Key(w[3..].join(" "), down.unwrap()),
            (Some("pad"), 4) => Host::Pad(w[3].into(), down.unwrap()),
            (Some("axis"), 5) => Host::Axis(w[2].into(), num(w[3])?, num(w[4])?),
            (Some("mouse"), 4) => Host::MouseMove(num(w[2])?, num(w[3])?),
            (Some("button"), 4) => Host::MouseButton(w[3] == "left", down.unwrap()),
            _ => return Err(bad()),
        };
        out.push((tick, e));
    }
    Ok(out)
}

/// An SDL event as the player's action, for the events the mapping knows.
pub fn host_event(e: &Event, sticks: &mut [i32; 6]) -> Option<Host> {
    let button = |b: &Button| {
        Some(match b {
            Button::A => "a",
            Button::B => "b",
            Button::X => "x",
            Button::Y => "y",
            Button::LeftShoulder => "lb",
            Button::RightShoulder => "rb",
            Button::Start => "start",
            Button::Back => "back",
            Button::LeftStick => "ls",
            Button::RightStick => "rs",
            Button::DPadUp => "dpad_up",
            Button::DPadDown => "dpad_down",
            Button::DPadLeft => "dpad_left",
            Button::DPadRight => "dpad_right",
            _ => return None,
        })
    };
    Some(match e {
        Event::KeyDown {
            scancode: Some(s),
            repeat: false,
            ..
        } => Host::Key(s.name().into(), true),
        Event::KeyUp {
            scancode: Some(s), ..
        } => Host::Key(s.name().into(), false),
        Event::ControllerButtonDown { button: b, .. } => Host::Pad(button(b)?.into(), true),
        Event::ControllerButtonUp { button: b, .. } => Host::Pad(button(b)?.into(), false),
        Event::ControllerAxisMotion { axis, value, .. } => {
            let i = *axis as usize;
            sticks[i.min(5)] = *value as i32;
            match axis {
                Axis::LeftX | Axis::LeftY => Host::Axis("left_stick".into(), sticks[0], sticks[1]),
                Axis::RightX | Axis::RightY => {
                    Host::Axis("right_stick".into(), sticks[2], sticks[3])
                }
                Axis::TriggerLeft => Host::Axis("lt".into(), sticks[4], 0),
                Axis::TriggerRight => Host::Axis("rt".into(), sticks[5], 0),
            }
        }
        Event::MouseMotion { xrel, yrel, .. } => Host::MouseMove(*xrel, *yrel),
        Event::MouseButtonDown { mouse_btn, .. } if *mouse_btn != MouseButton::Middle => {
            Host::MouseButton(*mouse_btn == MouseButton::Left, true)
        }
        Event::MouseButtonUp { mouse_btn, .. } if *mouse_btn != MouseButton::Middle => {
            Host::MouseButton(*mouse_btn == MouseButton::Left, false)
        }
        _ => return None,
    })
}

/// The largest rectangle of the frame's shape (Amiga low-res pixels shown square) centred in the window; the rest
/// is letterbox.
pub fn fit_rect(win_w: u32, win_h: u32) -> Rect {
    let (fw, fh) = (SCR_W as u64, SCR_H as u64);
    let (w, h) = if win_w as u64 * fh <= win_h as u64 * fw {
        (win_w as u64, win_w as u64 * fh / fw)
    } else {
        (win_h as u64 * fw / fh, win_h as u64)
    };
    Rect::new(
        ((win_w as u64 - w) / 2) as i32,
        ((win_h as u64 - h) / 2) as i32,
        w.max(1) as u32,
        h.max(1) as u32,
    )
}

fn sdl<T, E: ToString>(r: Result<T, E>) -> Result<T, String> {
    r.map_err(|e| e.to_string())
}

/// Runs the game until the player quits, or for `o.ticks` ticks.
pub fn run<F>(o: &Options, step: &mut F) -> Result<Outcome, String>
where
    F: FnMut(&mut Cpu, &mut GuestMemory) -> Result<(), Stop>,
{
    let ctx = sdl2::init()?;
    let video = ctx.video()?;
    let pads = ctx.game_controller()?;
    let mut win = video.window(&o.title, SCR_W as u32 * 3, SCR_H as u32 * 3);
    win.position_centered().resizable();
    if o.fullscreen {
        win.fullscreen_desktop();
    }
    let mut canvas = sdl(sdl(win.build())?.into_canvas().build())?;
    let tc = canvas.texture_creator();
    let mut tex =
        sdl(tc.create_texture_streaming(PixelFormatEnum::ARGB8888, SCR_W as u32, SCR_H as u32))?;
    let mut menu = Menu::new(&tc)?;
    // no audio device is not fatal: the game runs silent
    let audio = ctx.audio().and_then(|a| {
        let spec = sdl2::audio::AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(2),
            samples: Some(1024),
        };
        a.open_queue::<i16, _>(None, &spec)
    });
    if let Ok(q) = &audio {
        q.resume();
    }
    if o.controls.mouse && o.ticks.is_none() {
        ctx.mouse().set_relative_mouse_mode(true);
    }
    let mut events = ctx.event_pump()?;
    let mut open_pads = Vec::new();
    let mut sticks = [0i32; 6];
    let mut mapper = Mapper::new(o.controls.clone());
    let mut m = Machine::new(o.tree.boot(&o.program)?);
    let mut tick = 0u32;
    let mut paused = false;
    let mut bytes = vec![0u8; SCR_W * SCR_H * 4];
    let mut base = (Instant::now(), 0u32);
    loop {
        for e in events.poll_iter() {
            match &e {
                Event::Quit { .. } => return Ok(Outcome::Quit),
                Event::ControllerDeviceAdded { which, .. } => {
                    if let Ok(p) = pads.open(*which) {
                        open_pads.push(p);
                    }
                }
                _ => {}
            }
            let Some(h) = host_event(&e, &mut sticks) else {
                continue;
            };
            if paused {
                match menu.handle(&h) {
                    Some(MenuItem::Resume) => paused = false,
                    Some(MenuItem::Restart) => {
                        m = Machine::new(o.tree.boot(&o.program)?);
                        paused = false;
                    }
                    Some(MenuItem::Quit) => return Ok(Outcome::Quit),
                    None => {}
                }
            } else if mapper.handle(&h) {
                paused = true;
                mapper.release_all();
                menu.open();
            }
        }
        // the fixed timestep: as many ticks as the clock owes since `base`. Paused, or after a stall of more than a few
        // ticks, the clock is set again from now, so the game never runs fast to catch up.
        let due = match o.ticks {
            Some(_) => tick + 1,
            None => base.1 + (base.0.elapsed().as_micros() / TICK.as_micros()) as u32,
        };
        if paused || due > tick + 5 {
            base = (Instant::now(), tick);
        }
        while !paused && tick < due {
            tick += 1;
            for (_, h) in o.script.iter().filter(|(t, _)| *t == tick) {
                mapper.handle(h);
            }
            let input = mapper.tick_input();
            let keys = mapper.take_keys();
            m.chips(|c| {
                c.input = input;
                for (rk, down) in keys {
                    c.cias.push_key(rk, down);
                }
            });
            let r = m.run_tick(step);
            let (px, snd) = m.take_output();
            if let Ok(q) = &audio {
                if q.size() < AUDIO_MAX_TICKS * 882 * 4 {
                    sdl(q.queue_audio(&snd))?;
                }
            }
            if let Err(stop) = r {
                return Ok(Outcome::Stopped(tick - 1, format!("{stop:?}")));
            }
            if let Some(dir) = &o.frames {
                dump_frame(dir, tick, &px).map_err(|e| e.to_string())?;
            }
            for (b, p) in bytes.chunks_exact_mut(4).zip(&px) {
                b.copy_from_slice(&p.to_ne_bytes());
            }
            if o.ticks == Some(tick) {
                return Ok(Outcome::Ticks(tick));
            }
        }
        sdl(tex.update(None, &bytes, SCR_W * 4))?;
        canvas.set_draw_color(Color::RGB(0x14, 0x16, 0x1A));
        canvas.clear();
        let (ww, wh) = canvas.output_size()?;
        canvas.copy(&tex, None, Some(fit_rect(ww, wh)))?;
        if paused {
            menu.draw(&mut canvas)?;
        }
        canvas.present();
        if o.ticks.is_none() {
            let next = TICK * (tick + 1 - base.1);
            std::thread::sleep(next.saturating_sub(base.0.elapsed()).min(TICK));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_lines_parse() {
        let s = "# comment\n3 key down Right Ctrl\n4 pad up a\n5 axis left_stick -30000 0\n6 mouse 2 -1\n7 button down right\n";
        assert_eq!(
            parse_script(s).unwrap(),
            vec![
                (3, Host::Key("Right Ctrl".into(), true)),
                (4, Host::Pad("a".into(), false)),
                (5, Host::Axis("left_stick".into(), -30000, 0)),
                (6, Host::MouseMove(2, -1)),
                (7, Host::MouseButton(false, true)),
            ]
        );
        assert!(parse_script("x key down A").is_err());
        assert!(parse_script("1 wiggle").is_err());
    }

    #[test]
    fn frame_fits_the_window_centred() {
        assert_eq!(fit_rect(352 * 3, 272 * 3), Rect::new(0, 0, 1056, 816));
        // 1920x1080: full height, pillarbox left and right
        let r = fit_rect(1920, 1080);
        assert_eq!((r.height(), r.width(), r.x()), (1080, 1397, 261));
        // a tall phone in portrait: full width, letterbox top and bottom
        let r = fit_rect(1080, 2400);
        assert_eq!((r.width(), r.height(), r.y()), (1080, 834, 783));
    }

    #[test]
    fn sdl_events_become_host_events() {
        use sdl2::keyboard::Scancode;
        let mut st = [0; 6];
        let e = Event::KeyDown {
            timestamp: 0,
            window_id: 0,
            keycode: None,
            scancode: Some(Scancode::RCtrl),
            keymod: sdl2::keyboard::Mod::NOMOD,
            repeat: false,
        };
        assert_eq!(
            host_event(&e, &mut st),
            Some(Host::Key("Right Ctrl".into(), true))
        );
        let e = Event::ControllerAxisMotion {
            timestamp: 0,
            which: 0,
            axis: Axis::LeftY,
            value: -20000,
        };
        assert_eq!(
            host_event(&e, &mut st),
            Some(Host::Axis("left_stick".into(), 0, -20000))
        );
        let e = Event::ControllerButtonDown {
            timestamp: 0,
            which: 0,
            button: Button::DPadLeft,
        };
        assert_eq!(
            host_event(&e, &mut st),
            Some(Host::Pad("dpad_left".into(), true))
        );
    }
}
