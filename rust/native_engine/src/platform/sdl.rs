//! The SDL2 front end, shared by the desktop binary and the Android library: `app_main` reads the command line, finds
//! the asset pack (`--pack`, else one built into the binary, else the `gamepack.bin` asset, which on Android is the
//! APK asset), and either runs headless (`--ticks`), draws the pre-boot screenshot (`--preboot-shot`), or opens the
//! window: front door first, then the game at its fixed tick rate, drawn at the display's refresh rate.
//!
//!   GAME [--pack PACK] [--fullscreen] [--touch on|off] [--seed N]
//!        [--ticks N [--out DIR] [--every K] [--script FILE]] [--preboot-shot OUT.png [--preboot-at S]]

use std::io::Read;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use sdl2::controller::{Axis, Button};
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::audio::Mixer;
use crate::frontdoor::menu::{MenuItem, PauseMenu};
use crate::frontdoor::preboot::{nav_of, Item, Nav, Preboot, Screen};
use crate::frontdoor::theme::Canvas;
use crate::frontdoor::touch::{self, Phase, TouchPad};
use crate::game::{Ctx, Flow, Options, Spec};
use crate::gfx::Frame;
use crate::headless::{self, OUT_RATE};
use crate::input::{Host, Mapper};
use crate::pack::Pack;
use crate::save::Save;
use crate::timing::FixedStep;

pub const PACK_ASSET: &str = "gamepack.bin";
pub const USAGE: &str = "usage: GAME [--pack PACK] [--fullscreen] [--touch on|off] [--seed N] [--ticks N [--out DIR] [--every K] [--script FILE]] [--preboot-shot OUT.png [--preboot-at S]]";
/// Audio queued beyond this many ticks is dropped, so a slow host never lets the sound drift behind the picture.
const AUDIO_MAX_TICKS: u32 = 6;

fn get<'a>(v: &'a [String], k: &str) -> Option<&'a str> {
    v.iter()
        .position(|a| a == k)
        .and_then(|i| v.get(i + 1))
        .map(String::as_str)
}

fn read_asset(name: &str) -> Result<Vec<u8>, String> {
    let mut rw = sdl2::rwops::RWops::from_file(name, "rb").map_err(|e| format!("{name}: {e}"))?;
    let mut b = Vec::new();
    rw.read_to_end(&mut b).map_err(|e| format!("{name}: {e}"))?;
    Ok(b)
}

/// The whole program: returns its exit code (0 done, 2 bad arguments or files, 1 a platform failure).
pub fn app_main(v: &[String], embedded: Option<&[u8]>, spec: &Spec) -> i32 {
    match start(v, embedded, spec) {
        Ok(()) => 0,
        Err(e) => {
            let msg = format!("{}: {e}", spec.info.id);
            eprintln!("{msg}\n{USAGE}");
            sdl2::log::log(&msg);
            2
        }
    }
}

fn start(v: &[String], embedded: Option<&[u8]>, spec: &Spec) -> Result<(), String> {
    let pack = match (get(v, "--pack"), embedded) {
        (Some(p), _) if p.ends_with('/') || std::path::Path::new(p).is_dir() => {
            Pack::from_dir(p.as_ref())?
        }
        (Some(p), _) => Pack::from_bytes(&std::fs::read(p).map_err(|e| format!("{p}: {e}"))?)?,
        (None, Some(b)) => Pack::from_bytes(b)?,
        (None, None) => Pack::from_bytes(&read_asset(PACK_ASSET)?)?,
    };
    let opts = Options {
        seed: get(v, "--seed")
            .map(|s| s.parse().map_err(|_| "--seed must be a number"))
            .transpose()?,
    };
    let touch = match get(v, "--touch") {
        Some("on") => Some(true),
        Some("off") => Some(false),
        Some(_) => return Err("--touch must be on or off".into()),
        None => None,
    };
    if let Some(out) = get(v, "--preboot-shot") {
        let at: f32 = get(v, "--preboot-at")
            .unwrap_or("0")
            .parse()
            .map_err(|_| "--preboot-at must be seconds")?;
        return crate::frontdoor::preboot::shot(
            &spec.info.title,
            &spec.info.sheet,
            touch,
            at.clamp(0.0, 30.0),
            out.as_ref(),
        );
    }
    if let Some(t) = get(v, "--ticks") {
        let script = match get(v, "--script") {
            Some(p) => {
                crate::script::parse(&std::fs::read_to_string(p).map_err(|e| format!("{p}: {e}"))?)?
            }
            None => Vec::new(),
        };
        let r = headless::Run {
            ticks: t.parse().map_err(|_| "--ticks must be a number")?,
            out: get(v, "--out").map(PathBuf::from),
            every: get(v, "--every")
                .unwrap_or("1")
                .parse()
                .map_err(|_| "--every must be a number")?,
            script,
        };
        let n = headless::run(spec, &pack, &opts, &r)?;
        eprintln!("{}: ran {n} ticks", spec.info.id);
        return Ok(());
    }
    sdl2::hint::set("SDL_ANDROID_TRAP_BACK_BUTTON", "1");
    let fullscreen = cfg!(target_os = "android") || v.iter().any(|a| a == "--fullscreen");
    window(spec, &pack, &opts, fullscreen, touch)
}

/// An SDL event as a host event, for the events the mapping knows (touch is handled separately).
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
        Event::MouseMotion {
            which, xrel, yrel, ..
        } if *which != u32::MAX => Host::MouseMove(*xrel, *yrel),
        Event::MouseButtonDown {
            which, mouse_btn, ..
        } if *which != u32::MAX && *mouse_btn != MouseButton::Middle => {
            Host::MouseButton(*mouse_btn == MouseButton::Left, true)
        }
        Event::MouseButtonUp {
            which, mouse_btn, ..
        } if *which != u32::MAX && *mouse_btn != MouseButton::Middle => {
            Host::MouseButton(*mouse_btn == MouseButton::Left, false)
        }
        _ => return None,
    })
}

/// The largest rectangle of the game's shape (square pixels) centred in the window; the rest is letterbox.
pub fn fit_rect(win_w: u32, win_h: u32, fw: usize, fh: usize) -> Rect {
    let (fw, fh) = (fw as u64, fh as u64);
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

/// Shows a CPU-drawn picture (pre-boot, overlays) over the whole window or in `dst`.
fn blit<'a>(
    canvas: &mut WindowCanvas,
    tc: &'a TextureCreator<WindowContext>,
    slot: &mut Option<Texture<'a>>,
    cv: &Canvas,
    dst: Option<Rect>,
) -> Result<(), String> {
    let (w, h) = (cv.w as u32, cv.h as u32);
    if !slot
        .as_ref()
        .is_some_and(|t| (t.query().width, t.query().height) == (w, h))
    {
        let mut t = sdl(tc.create_texture_streaming(PixelFormatEnum::ARGB8888, w, h))?;
        t.set_blend_mode(BlendMode::Blend);
        *slot = Some(t);
    }
    let tex = slot.as_mut().ok_or("no texture")?;
    let b: Vec<u8> = cv.px.iter().flat_map(|p| p.to_ne_bytes()).collect();
    sdl(tex.update(None, &b, cv.w * 4))?;
    canvas.copy(tex, None, dst)
}

fn window(
    spec: &Spec,
    pack: &Pack,
    opts: &Options,
    fullscreen: bool,
    touch_arg: Option<bool>,
) -> Result<(), String> {
    let info = &spec.info;
    let (lw, lh) = (info.width, info.height);
    let ctx = sdl2::init()?;
    let video = ctx.video()?;
    let pads = ctx.game_controller()?;
    let mut win = video.window(&info.title, lw as u32 * 3, lh as u32 * 3);
    win.position_centered().resizable();
    if fullscreen {
        win.fullscreen_desktop();
    }
    let mut canvas = sdl(sdl(win.build())?.into_canvas().present_vsync().build())?;
    let tc = canvas.texture_creator();
    let mut tex =
        sdl(tc.create_texture_streaming(PixelFormatEnum::ARGB8888, lw as u32, lh as u32))?;
    let audio = ctx.audio().and_then(|a| {
        let spec = sdl2::audio::AudioSpecDesired {
            freq: Some(OUT_RATE as i32),
            channels: Some(2),
            samples: Some(1024),
        };
        a.open_queue::<i16, _>(None, &spec)
    });
    if let Ok(q) = &audio {
        q.resume();
    }
    let mut events = ctx.event_pump()?;
    let mut open_pads = Vec::new();
    let mut sticks = [0i32; 6];
    let store = sdl2::filesystem::pref_path("Ami-Go", &format!("native_{}", info.id))
        .ok()
        .map(PathBuf::from);
    let mut save = store.clone().map(Save::open).unwrap_or_else(Save::memory);
    // the front door (Karl, 14 Oct 2026: the pre-boot at every start)
    let mut pre = Preboot::new(&info.title, &info.sheet)?;
    let android = cfg!(target_os = "android") && touch_arg.is_none();
    pre.touch = touch_arg.or_else(|| {
        android.then(|| {
            let n = pads.num_joysticks().unwrap_or(0);
            let connected = (0..n).filter(|&i| pads.is_game_controller(i)).count();
            let screen = sdl2::touch::num_touch_devices() > 0;
            store
                .as_deref()
                .and_then(touch::load)
                .unwrap_or_else(|| touch::default_on(screen, connected))
        })
    });
    let mut saved = pre.touch;
    let mut full = None;
    let mut last = Instant::now();
    loop {
        let (ww, wh) = canvas.output_size()?;
        let (vw, vh) = canvas.window().size();
        let at = |x: i32, y: i32| {
            let (fx, fy) = (ww as f32 / vw.max(1) as f32, wh as f32 / vh.max(1) as f32);
            Preboot::to_design(ww as usize, wh as usize, x as f32 * fx, y as f32 * fy)
        };
        let mut picked = None;
        for e in events.poll_iter() {
            let n = match &e {
                Event::Quit { .. } => return Ok(()),
                Event::ControllerDeviceAdded { which, .. } => {
                    if let Ok(p) = pads.open(*which) {
                        open_pads.push(p);
                    }
                    None
                }
                Event::MouseMotion { x, y, which, .. } if *which != u32::MAX => {
                    Some(Nav::Point(at(*x, *y).0, at(*x, *y).1))
                }
                Event::MouseButtonDown { x, y, which, .. } if *which != u32::MAX => {
                    Some(Nav::Click(at(*x, *y).0, at(*x, *y).1))
                }
                Event::FingerDown { x, y, .. } => {
                    let d =
                        Preboot::to_design(ww as usize, wh as usize, x * ww as f32, y * wh as f32);
                    Some(Nav::Click(d.0, d.1))
                }
                _ => host_event(&e, &mut sticks).and_then(|h| nav_of(&h)),
            };
            picked = picked.or(n.and_then(|n| pre.handle(n)));
        }
        if let (Some(d), true) = (&store, pre.touch != saved) {
            saved = pre.touch;
            let _ = touch::save(d, pre.touch == Some(true));
        }
        let ms = last.elapsed().as_millis() as u32;
        last += Duration::from_millis(ms as u64);
        match picked.or(pre.advance(ms)) {
            Some(Item::Quit) => return Ok(()),
            Some(_) => break,
            None => {}
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        blit(
            &mut canvas,
            &tc,
            &mut full,
            &pre.render(ww as usize, wh as usize),
            None,
        )?;
        canvas.present();
    }
    // the game
    let mut game = (spec.make)(pack, opts)?;
    let mut mapper = Mapper::new(info.bindings.clone());
    let mut mixer = Mixer::new(OUT_RATE, info.tick_hz, info.effect_voices);
    let mut frame = Frame::new(lw, lh);
    let mut argb = Vec::new();
    let mut bytes = vec![0u8; lw * lh * 4];
    let mut menu = PauseMenu::new()?;
    let mut menu_tex = None;
    let mut over = None;
    let (ww, wh) = canvas.output_size()?;
    let mut tpad = match pre.touch {
        Some(true) => Some(TouchPad::new(&info.sheet, ww, wh)?),
        _ => None,
    };
    let mut paused = false;
    let mut show_controls = false;
    let mut relative = false;
    let mut clock = FixedStep::new(info.tick_hz);
    let mut tick = 0u32;
    let per_tick = (OUT_RATE / info.tick_hz.max(1)) as usize;
    let mut snd = Vec::new();
    let (mut fps_n, mut fps_t) = (0u32, Instant::now());
    loop {
        let (ww, wh) = canvas.output_size()?;
        let (vw, vh) = canvas.window().size();
        let fit = fit_rect(ww, wh, lw, lh);
        let to_logical = move |x: f32, y: f32| {
            let lx = ((x - fit.x() as f32) * lw as f32 / fit.width() as f32).floor() as i32;
            let ly = ((y - fit.y() as f32) * lh as f32 / fit.height() as f32).floor() as i32;
            (lx >= 0 && ly >= 0 && lx < lw as i32 && ly < lh as i32).then_some((lx, ly))
        };
        if let Some(t) = tpad.as_mut().filter(|t| t.size() != (ww, wh)) {
            t.resize(ww, wh);
            mapper.release_all();
        }
        for e in events.poll_iter() {
            match &e {
                Event::Quit { .. } => return Ok(()),
                Event::ControllerDeviceAdded { which, .. } => {
                    if let Ok(p) = pads.open(*which) {
                        open_pads.push(p);
                    }
                }
                _ => {}
            }
            let tap = match &e {
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    which,
                    ..
                } if *which != u32::MAX => Some((
                    *x as f32 * ww as f32 / vw.max(1) as f32,
                    *y as f32 * wh as f32 / vh.max(1) as f32,
                )),
                Event::FingerDown { x, y, .. } => Some((x * ww as f32, y * wh as f32)),
                _ => None,
            };
            let finger = match &e {
                Event::FingerDown {
                    finger_id, x, y, ..
                } => Some((*finger_id, Phase::Down, *x, *y)),
                Event::FingerMotion {
                    finger_id, x, y, ..
                } => Some((*finger_id, Phase::Move, *x, *y)),
                Event::FingerUp {
                    finger_id, x, y, ..
                } => Some((*finger_id, Phase::Up, *x, *y)),
                _ => None,
            };
            if show_controls {
                let any = finger.is_some_and(|f| f.1 == Phase::Down)
                    || host_event(&e, &mut sticks).is_some_and(|h| {
                        nav_of(&h).is_some() || matches!(h, Host::MouseButton(_, true))
                    });
                show_controls = !any;
                continue;
            }
            if paused {
                let pick = match (tap, host_event(&e, &mut sticks)) {
                    (Some((x, y)), _) => menu.hit(ww, wh, x, y),
                    (None, Some(h)) => menu.handle(&h),
                    _ => None,
                };
                match pick {
                    Some(MenuItem::Resume) => paused = false,
                    Some(MenuItem::Restart) => {
                        game = (spec.make)(pack, opts)?;
                        mixer = Mixer::new(OUT_RATE, info.tick_hz, info.effect_voices);
                        paused = false;
                    }
                    Some(MenuItem::Controls) => {
                        pre.screen = Screen::Controls;
                        show_controls = true;
                    }
                    Some(MenuItem::Quit) => return Ok(()),
                    None => {}
                }
                if !paused {
                    clock.reset(Instant::now());
                }
                continue;
            }
            let hosts: Vec<Host> = match (finger, tpad.as_mut()) {
                (Some((id, ph, x, y)), Some(t)) => {
                    t.finger(id, ph, x * ww as f32, y * wh as f32, to_logical)
                }
                (Some((id, ph, x, y)), None) if info.sheet.touch_pointer => {
                    // no on-screen controls: a finger is still the pointer in a pointer game
                    let mut v = vec![];
                    if let Some((lx, ly)) = to_logical(x * ww as f32, y * wh as f32) {
                        v.push(Host::PointerAt(lx, ly));
                    }
                    match ph {
                        Phase::Down => v.push(Host::Touch(
                            format!("finger{id}"),
                            crate::input::Action::Primary,
                            true,
                        )),
                        Phase::Up => v.push(Host::Touch(
                            format!("finger{id}"),
                            crate::input::Action::Primary,
                            false,
                        )),
                        Phase::Move => {}
                    }
                    v
                }
                (Some(_), None) => vec![],
                (None, _) => host_event(&e, &mut sticks).into_iter().collect(),
            };
            for h in hosts {
                if mapper.handle(&h) {
                    paused = true;
                    mapper.release_all();
                    if let Some(t) = &mut tpad {
                        t.release_all();
                    }
                    menu.open();
                }
            }
        }
        let want = !paused && info.bindings.mouse && !cfg!(target_os = "android");
        if want != relative {
            ctx.mouse().set_relative_mouse_mode(want);
            relative = want;
        }
        if !paused && !show_controls {
            let n = clock.due(Instant::now());
            for _ in 0..n {
                tick += 1;
                let input = mapper.tick();
                let flow = game.tick(&mut Ctx {
                    input: &input,
                    audio: &mut mixer,
                    save: &mut save,
                    tick,
                });
                snd.clear();
                mixer.render(per_tick, &mut snd);
                if let Ok(q) = &audio {
                    if q.size() < AUDIO_MAX_TICKS * per_tick as u32 * 4 {
                        sdl(q.queue_audio(&snd))?;
                    }
                }
                clock.ran();
                if flow == Flow::Quit {
                    return Ok(());
                }
            }
        }
        game.render(&mut frame);
        frame.to_argb(&mut argb);
        for (b, p) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(&argb) {
            *b = p.to_ne_bytes();
        }
        sdl(tex.update(None, &bytes, lw * 4))?;
        canvas.set_draw_color(Color::RGB(0x14, 0x16, 0x1A));
        canvas.clear();
        canvas.copy(&tex, None, Some(fit))?;
        if show_controls {
            blit(
                &mut canvas,
                &tc,
                &mut full,
                &pre.render(ww as usize, wh as usize),
                None,
            )?;
        } else if paused {
            let (x, y, w, h) = PauseMenu::place(ww, wh);
            blit(
                &mut canvas,
                &tc,
                &mut menu_tex,
                &menu.render(),
                Some(Rect::new(x, y, w, h)),
            )?;
        } else if let Some(t) = &mut tpad {
            if t.dirty || over.is_none() {
                t.dirty = false;
                blit(&mut canvas, &tc, &mut over, &t.render(), None)?;
            } else if let Some(x) = &over {
                canvas.copy(x, None, None)?;
            }
        }
        canvas.present();
        fps_n += 1;
        if fps_t.elapsed() >= Duration::from_secs(5) {
            let fps = fps_n as f32 / fps_t.elapsed().as_secs_f32();
            sdl2::log::log(&format!(
                "native {}: {fps:.1} fps drawn, tick {tick}",
                info.id
            ));
            fps_n = 0;
            fps_t = Instant::now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_fits_the_window_centred() {
        assert_eq!(fit_rect(960, 768, 320, 256), Rect::new(0, 0, 960, 768));
        let r = fit_rect(1920, 1080, 320, 256);
        assert_eq!((r.height(), r.width(), r.x()), (1080, 1350, 285));
        let r = fit_rect(1080, 2400, 320, 256);
        assert_eq!((r.width(), r.height(), r.y()), (1080, 864, 768));
    }
}
