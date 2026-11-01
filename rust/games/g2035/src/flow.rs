//! The game's top-level flow: the phases of the recovered start-up and main programs in order. The loading wait
//! after the intro (the original read the main program off disk 1 and asked for disk 2, showing its loading
//! pictures) is not ported, as nothing is read from disks; the language screen follows the intro directly.
//!
//! The front end follows the main program's front-end routine ($C1D25E): the language screen in attract mode (after
//! 1500 idle frames it shows the main menu, after 375 more it plays a computer v computer friendly, then the
//! language screen again with the shorter 375-frame time-out), then the main menu, whose choices dispatch through
//! the table at $C1D332.

use native_engine::game::{Ctx, Flow, Game, Options};
use native_engine::gfx::Frame;
use native_engine::input::Action;

use crate::assets::Assets;
use crate::intro::Intro;
use crate::menu::{self, Event, MenuRun, Stick, LANGUAGE, MAIN};

/// Fade level (sixteenths) per frame after a menu's result.
const FADE_OUT: [u32; 9] = [16, 16, 8, 4, 2, 0, 0, 0, 0];

/// Which call of the front-end routine a menu is shown from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Site {
    /// The language screen at start-up and after the attract demo ($C1D2A2).
    Language,
    /// The main menu shown by the language screen's time-out, still in attract mode ($C1D2C2).
    MainAttract,
    /// The language screen after fire on the attract main menu ($C1D2E0), no time-out.
    LanguageAgain,
    /// The main menu proper ($C1D306).
    Main,
}

pub enum Phase {
    Intro(Intro),
    Menu(MenuRun, Site),
    /// After a menu's result: the cursor is erased, the screen fades out (frames 2 to 4) and stays black until
    /// the next screen opens at frame 10 (measured on the reference language -> main menu change).
    FadeOut {
        run: MenuRun,
        site: Site,
        ev: Event,
        t: u32,
    },
    /// A main menu choice whose screens come in later parts (Options, Edit Teams, Load/Save, Highlights, Friendly,
    /// Cup, League, Specials): a black screen for now; back or fire returns to the main menu.
    Pending {
        item: usize,
        ticks: u32,
    },
}

pub struct Native {
    pub a: Assets,
    pub phase: Phase,
    pub seed: u32,
    /// 0 English, 1 Deutsch, 2 Francais, 3 Italiano ($C1FE74).
    pub language: usize,
    /// The attract time-out ($C1D372): 1500 frames at first, 375 once the language screen has returned.
    pub idle_limit: u32,
    /// Where the main menu's cursor was when it last dispatched (kept in its descriptor by the original).
    pub main_at: (usize, i32, i32),
    /// The frame counter ($C0DE6A) between menus: it runs on through fades and the screens in between.
    pub counter: u32,
}

impl Native {
    pub fn new(a: Assets, o: &Options) -> Native {
        let m = &a.front.menus[MAIN];
        let main_at = (m.first, m.col, m.row);
        let idle_limit = a.front.ui.language_idle;
        Native {
            a,
            phase: Phase::Intro(Intro::new()),
            seed: o.seed.unwrap_or(0),
            language: 0,
            idle_limit,
            main_at,
            counter: 0,
        }
    }

    /// The language screen; the counter is cleared at $C1D2AA (start-up, after the demo) but not at $C1D2E0.
    fn open_language(&mut self, attract: bool, site: Site) {
        let c = if site == Site::Language {
            0
        } else {
            self.counter
        };
        let r = MenuRun::open(&self.a.front, LANGUAGE, attract, self.idle_limit, c);
        self.phase = Phase::Menu(r, site);
    }

    /// The main menu; the counter is cleared only for the attract one ($C1D2CA).
    fn open_main(&mut self, attract: bool, site: Site) {
        let c = if site == Site::MainAttract {
            0
        } else {
            self.counter
        };
        let mut r = MenuRun::reopen(&self.a.front, MAIN, self.main_at, c);
        r.attract = attract;
        r.idle_limit = self.idle_limit;
        self.phase = Phase::Menu(r, site);
    }

    /// What a menu's result leads to.
    fn menu_done(&mut self, site: Site, ev: Event, run: &MenuRun) {
        let short = self.a.front.ui.main_idle;
        match (site, ev) {
            (Site::Language, Event::Idle) => {
                self.idle_limit = short;
                self.open_main(true, Site::MainAttract);
            }
            (Site::Language | Site::LanguageAgain, Event::Activated(i)) => {
                self.idle_limit = short;
                self.language = i.min(3);
                self.open_main(false, Site::Main);
            }
            // the computer v computer demo friendly comes with the match engine; until then straight back
            (Site::MainAttract, Event::Idle) => self.open_language(true, Site::Language),
            (Site::MainAttract, Event::AttractExit) => {
                self.open_language(false, Site::LanguageAgain)
            }
            (_, Event::Activated(i)) => {
                self.main_at = (run.cur, run.col, run.row);
                self.phase = Phase::Pending { item: i, ticks: 0 };
            }
            _ => {}
        }
    }
}

fn stick(cx: &Ctx) -> Stick {
    let h = |a| cx.input.held(a);
    Stick {
        left: h(Action::Left),
        right: h(Action::Right),
        up: h(Action::Up),
        down: h(Action::Down),
        // Return counts as fire in the original's menus (key code $0D, $C1E4D8)
        fire: h(Action::Fire) || h(Action::Start),
    }
}

impl Game for Native {
    fn tick(&mut self, cx: &mut Ctx) -> Flow {
        let s = stick(cx);
        match &mut self.phase {
            Phase::Intro(i) => {
                let skip = cx.input.pressed(Action::Fire) || cx.input.pressed(Action::Start);
                if i.tick(&self.a.intro, skip) {
                    self.open_language(true, Site::Language);
                }
            }
            Phase::Menu(run, site) => {
                let site = *site;
                let ev = run.tick(&self.a.front, s);
                self.counter = run.ticks;
                if let Event::Activated(_) = ev {
                    let run = run.clone();
                    self.phase = Phase::FadeOut {
                        run,
                        site,
                        ev,
                        t: 0,
                    };
                } else if ev != Event::None {
                    // attract changes: no fade (the fades belong to the activation path; the attract path is
                    // not yet checked against a reference, whose snapshot starts with attract mode already off)
                    let run = run.clone();
                    self.menu_done(site, ev, &run);
                }
            }
            Phase::FadeOut { run, site, ev, t } => {
                *t += 1;
                self.counter += 1;
                if *t >= FADE_OUT.len() as u32 {
                    let (run, site, ev) = (run.clone(), *site, *ev);
                    self.menu_done(site, ev, &run);
                }
            }
            Phase::Pending { ticks, .. } => {
                *ticks += 1;
                self.counter += 1;
                if cx.input.pressed(Action::Cancel) || cx.input.pressed(Action::Fire) {
                    self.open_main(false, Site::Main);
                }
            }
        }
        Flow::Continue
    }

    fn render(&self, frame: &mut Frame) {
        match &self.phase {
            Phase::Intro(i) => i.render(&self.a.intro, frame),
            Phase::Menu(run, _) => menu::render(&self.a.front, run, self.language, frame),
            Phase::FadeOut { run, t, .. } => {
                let m = FADE_OUT[(*t as usize).min(FADE_OUT.len() - 1)];
                menu::render_screen(&self.a.front, run, self.language, frame, m, false);
            }
            Phase::Pending { .. } => {
                frame.palette.colours[0] = 0xFF00_0000;
                frame.image.fill(0);
            }
        }
    }

    /// phase 0 intro, 1 menu, 2 pending, 3 fading out; frames in the phase; menu 0 language, 1 main; cursor item;
    /// language; the frame counter.
    fn state(&self) -> Vec<(String, i64)> {
        let (p, t, m, c) = match &self.phase {
            Phase::Intro(i) => (0, i.ticks, -1, -1),
            Phase::Menu(r, _) => (1, r.age, r.menu as i64, r.cur as i64),
            Phase::FadeOut { run, t, .. } => (3, *t, run.menu as i64, run.cur as i64),
            Phase::Pending { item, ticks } => (2, *ticks, MAIN as i64, *item as i64),
        };
        vec![
            ("phase".into(), p),
            ("phase_ticks".into(), t as i64),
            ("menu".into(), m),
            ("cursor".into(), c),
            ("language".into(), self.language as i64),
            ("counter".into(), self.counter as i64),
        ]
    }
}
