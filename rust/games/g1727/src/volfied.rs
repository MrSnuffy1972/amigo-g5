//! The main program (+2C04) as typed states: the title and options, the "PLAYER 1 / ROUND nn" card, the round
//! loop, the round clear with the full picture, and game over. The HUD is drawn as the original places it (the
//! percentage at (288, 192) in colour 7 by +7DC8, the shield digits at (312, 120) in colour 3 by +6484, the lives at
//! (312, 160) in colour 7 by +7D4C).

use crate::assets::{Assets, H, ROUNDS, W};
use crate::field::{Cell, FH, FW};
use crate::play::{Dir, Outcome, Round};
use crate::rng::Rng;
use native_engine::game::{Ctx, Flow, Game, Options};
use native_engine::gfx::{Frame, Palette, Rect, TextMode};
use native_engine::input::Action;

pub const LIVES: u8 = 3;
const DIFFICULTIES: [&str; 4] = ["trainer", "easy", "normal", "hard"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Scene {
    Title,
    Card(u32),
    Play,
    Clear(u32),
    Over(u32),
}

pub struct Native {
    a: Assets,
    rng: Rng,
    scene: Scene,
    round: Round,
    level: usize,
    menu: usize,
    start_level: usize,
    difficulty: usize,
    lives: u8,
    score: u32,
    hiscore: u32,
}

impl Native {
    pub fn new(a: Assets, o: &Options) -> Native {
        let mut rng = Rng::new(o.seed.unwrap_or(0x2C04));
        let round = Round::new(0, &mut rng);
        Native {
            a,
            rng,
            scene: Scene::Title,
            round,
            level: 0,
            menu: 0,
            start_level: 0,
            difficulty: 2,
            lives: LIVES,
            score: 0,
            hiscore: 6_186_640,
        }
    }

    fn start_round(&mut self) {
        self.round = Round::new(self.level, &mut self.rng);
        self.scene = Scene::Card(100);
    }

    fn title_tick(&mut self, cx: &Ctx) {
        let i = cx.input;
        if i.pressed(Action::Up) && self.menu > 0 {
            self.menu -= 1;
        }
        if i.pressed(Action::Down) && self.menu < 1 {
            self.menu += 1;
        }
        let step = i.pressed(Action::Right) as i32 - i.pressed(Action::Left) as i32;
        if step != 0 {
            if self.menu == 0 {
                self.start_level =
                    (self.start_level as i32 + step).clamp(0, ROUNDS as i32 - 1) as usize;
            } else {
                self.difficulty = (self.difficulty as i32 + step).clamp(0, 3) as usize;
            }
        }
        if i.pressed(Action::Fire) || i.pressed(Action::Start) {
            self.level = self.start_level;
            self.lives = LIVES;
            self.score = 0;
            self.start_round();
        }
    }

    fn play_tick(&mut self, cx: &Ctx) {
        let i = cx.input;
        let dir = [
            (Action::Right, Dir::Right),
            (Action::Down, Dir::Down),
            (Action::Left, Dir::Left),
            (Action::Up, Dir::Up),
        ]
        .iter()
        .find(|(a, _)| i.held(*a))
        .map(|&(_, d)| d);
        let fire = i.held(Action::Fire);
        let mut out = Outcome::Playing;
        // the player routine runs twice a frame (measured: one pixel a frame, the shield one step in ten frames)
        for _ in 0..2 {
            if out == Outcome::Playing {
                out = self.round.player_step(dir, fire, false);
            }
        }
        if out == Outcome::Playing {
            out = self.round.enemies_step();
        }
        self.score += std::mem::take(&mut self.round.score_gain);
        self.hiscore = self.hiscore.max(self.score);
        match out {
            Outcome::Cleared => self.scene = Scene::Clear(250),
            Outcome::LifeLost => {
                self.round.lose_life();
                self.lives = self.lives.saturating_sub(1);
                if self.lives == 0 {
                    self.scene = Scene::Over(250);
                }
            }
            Outcome::Playing => {}
        }
        if i.pressed(Action::Cancel) {
            self.scene = Scene::Title;
        }
    }

    fn text(&self, f: &mut Frame, x: i32, y: i32, s: &str, c: u8) {
        self.a.font.draw(&mut f.image, x, y, s, TextMode::Ink(c));
    }

    fn draw_field(&self, f: &mut Frame, reveal: bool) {
        let r = self.round.number.min(ROUNDS - 1);
        let surface = &self.a.screens[r];
        let picture = &self.a.screens[r + 1];
        f.palette = if reveal {
            picture.palette.clone()
        } else {
            surface.palette.clone()
        };
        for y in 0..FH {
            for x in 0..FW {
                let c = match self.round.field.get(x, y) {
                    _ if reveal => picture.image.get(x, y - 1),
                    Cell::Free => surface.image.get(x, y - 1),
                    // the claimed area shows the picture's planes 0 and 1 (four greys)
                    Cell::Claimed => picture.image.get(x, y - 1) & 3,
                    Cell::Line => 7,
                    Cell::Cut => 15,
                };
                f.image.set(x, y, c);
            }
        }
        if reveal {
            return;
        }
        let clip = Rect::new(1, 1, FW - 2, FH - 2);
        let bobs = &self.a.bobs[r + 1];
        for e in &self.round.enemies {
            let k = if e.boss {
                0
            } else {
                bobs.rects.len().saturating_sub(1).min(7)
            };
            if let Some(&s) = bobs.rects.get(k) {
                f.image
                    .blit_keyed(&bobs.atlas, 15, s, e.x - s.w / 2, e.y - s.h / 2, clip);
            }
        }
        let p = &self.round.player;
        let c = if p.shield_on { 6 } else { 4 };
        for (dx, dy) in [
            (0, -2),
            (-2, 0),
            (2, 0),
            (0, 2),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ] {
            f.image.set(p.x + dx, p.y + dy, c);
        }
    }

    fn draw_hud(&self, f: &mut Frame) {
        let digits = |n: u32, w: usize| format!("{:0w$}", n, w = w);
        let score = digits(self.score, 7);
        let hi = digits(self.hiscore, 7);
        for (k, ch) in hi.chars().enumerate() {
            self.text(f, 320, 2 + 8 * k as i32, &ch.to_string(), 6);
        }
        self.text(f, 320, 58, "HI", 8);
        for (k, ch) in score.chars().enumerate() {
            self.text(f, 312, 34 + 8 * k as i32, &ch.to_string(), 6);
        }
        self.text(f, 312, 90, "1UP", 5);
        self.text(f, 288, 112, "SHIELD", 6);
        self.text(f, 312, 120, &digits(self.round.player.shield as u32, 3), 3);
        self.text(f, 296, 128, "ROUND", 6);
        self.text(f, 312, 136, &digits(self.round.number as u32 + 1, 2), 3);
        self.text(f, 296, 152, "LIVES", 6);
        self.text(f, 312, 160, &digits(self.lives as u32, 2), 7);
        let p = self.round.field.percent_digits();
        self.text(f, 288, 192, &format!("{}{}.{}", p[0], p[1], p[2]), 7);
        self.text(f, 320, 184, "%", 7);
    }
}

impl Game for Native {
    fn tick(&mut self, cx: &mut Ctx) -> Flow {
        self.rng.frames = self.rng.frames.wrapping_add(1);
        match self.scene {
            Scene::Title => self.title_tick(cx),
            Scene::Card(t) => {
                self.scene = if t <= 1 || cx.input.pressed(Action::Fire) {
                    Scene::Play
                } else {
                    Scene::Card(t - 1)
                }
            }
            Scene::Play => self.play_tick(cx),
            Scene::Clear(t) => {
                if t <= 1 {
                    self.level = (self.level + 1) % ROUNDS;
                    self.start_round();
                } else {
                    self.scene = Scene::Clear(t - 1);
                }
            }
            Scene::Over(t) => {
                self.scene = if t <= 1 || cx.input.pressed(Action::Fire) {
                    Scene::Title
                } else {
                    Scene::Over(t - 1)
                }
            }
        }
        Flow::Continue
    }

    fn render(&self, f: &mut Frame) {
        f.image.fill(0);
        match self.scene {
            Scene::Title => {
                f.palette = self.a.title.palette.clone();
                let t = &self.a.title.image;
                f.image
                    .blit(t, t.rect(), 0, 0, Rect::new(0, 0, W as i32, H as i32));
                let lvl = format!("{} {:02}", self.a.text("start_level"), self.start_level + 1);
                let dif = format!(
                    "Difficulty {}",
                    self.a.text(DIFFICULTIES[self.difficulty]).trim_end()
                );
                self.text(f, 88, 168, &lvl, if self.menu == 0 { 7 } else { 6 });
                self.text(f, 88, 178, &dif, if self.menu == 1 { 7 } else { 6 });
            }
            Scene::Card(_) => {
                f.palette = card_palette();
                let r = self
                    .a
                    .text("round")
                    .replace("??", &format!("{:02}", self.level + 1));
                self.text(f, 128, 88, &self.a.text("player").replace('?', "1"), 1);
                self.text(f, 128, 104, &r, 1);
            }
            Scene::Play | Scene::Over(_) => {
                self.draw_field(f, false);
                self.draw_hud(f);
                if matches!(self.scene, Scene::Over(_)) {
                    self.text(f, 108, 96, "GAME OVER", 7);
                }
            }
            Scene::Clear(_) => {
                self.draw_field(f, true);
                let r = self
                    .a
                    .text("round")
                    .replace("??", &format!("{:02}", self.level + 1));
                self.text(f, 100, 96, &format!("{r} CLEAR"), 7);
            }
        }
    }

    fn state(&self) -> Vec<(String, i64)> {
        let p = &self.round.player;
        vec![
            ("x".into(), p.x as i64),
            ("y".into(), p.y as i64),
            ("shield".into(), p.shield as i64),
            ("claimed".into(), self.round.field.count as i64),
            ("round".into(), self.round.number as i64),
            ("lives".into(), self.lives as i64),
        ]
    }
}

fn card_palette() -> Palette {
    let mut p = Palette::new(16);
    p.colours[1] = 0xFFFF_FFFF;
    p
}
