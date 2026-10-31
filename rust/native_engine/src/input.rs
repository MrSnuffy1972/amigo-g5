//! Input as abstract actions and a pointer. The player's pad, keyboard, mouse, touchscreen and TV remote arrive as
//! `Host` events; `Bindings` (data, per game) say what each control does; `Mapper` turns the events into one
//! `InputState` a tick: which actions are held, which were pressed or released since the last tick, and how the
//! pointer moved (relative motion, or an absolute position from a touch or a mouse that is not captured).

/// What a control does in a game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Fire,
    Fire2,
    Start,
    /// Leave or go back (the game's own "escape").
    Cancel,
    /// The pointer's main button (click, tap).
    Primary,
    /// The pointer's second button.
    Secondary,
    /// Opens the engine's pause menu; never reaches the game.
    Pause,
}

pub const ACTIONS: [Action; 11] = [
    Action::Up,
    Action::Down,
    Action::Left,
    Action::Right,
    Action::Fire,
    Action::Fire2,
    Action::Start,
    Action::Cancel,
    Action::Primary,
    Action::Secondary,
    Action::Pause,
];

impl Action {
    pub fn bit(self) -> u32 {
        1 << ACTIONS.iter().position(|&a| a == self).unwrap_or(0)
    }

    pub fn parse(s: &str) -> Option<Action> {
        ACTIONS.iter().copied().find(|a| a.name() == s)
    }

    pub fn name(self) -> &'static str {
        match self {
            Action::Up => "up",
            Action::Down => "down",
            Action::Left => "left",
            Action::Right => "right",
            Action::Fire => "fire",
            Action::Fire2 => "fire2",
            Action::Start => "start",
            Action::Cancel => "cancel",
            Action::Primary => "primary",
            Action::Secondary => "secondary",
            Action::Pause => "pause",
        }
    }
}

/// The input of one tick.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InputState {
    pub held: u32,
    pub pressed: u32,
    pub released: u32,
    /// Pointer motion since the last tick, in logical pixels.
    pub motion: (i32, i32),
    /// The pointer's absolute position this tick (a touch, or an uncaptured mouse), in logical pixels.
    pub absolute: Option<(i32, i32)>,
}

impl InputState {
    pub fn held(&self, a: Action) -> bool {
        self.held & a.bit() != 0
    }
    pub fn pressed(&self, a: Action) -> bool {
        self.pressed & a.bit() != 0
    }
    pub fn released(&self, a: Action) -> bool {
        self.released & a.bit() != 0
    }
}

/// What the player did, in the host's terms.
#[derive(Debug, Clone, PartialEq)]
pub enum Host {
    /// A key by SDL scancode name ("Escape", "Space", "Up", "AC Back" is the TV remote's Back).
    Key(String, bool),
    /// A pad button by name: a b x y lb rb start back ls rs dpad_up dpad_down dpad_left dpad_right.
    Pad(String, bool),
    /// A stick or trigger by name ("left_stick", "right_stick", "lt", "rt") at -32768..32767.
    Axis(String, i32, i32),
    /// Relative mouse motion in logical pixels.
    MouseMove(i32, i32),
    /// Left (true) or right mouse button, down or up.
    MouseButton(bool, bool),
    /// The pointer placed at a logical position (touch on a pointer game, an uncaptured mouse).
    PointerAt(i32, i32),
    /// An action held or let go by an on-screen control, by its own source name.
    Touch(String, Action, bool),
}

/// What a control is bound to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bind {
    Act(Action),
    /// A stick or the D-pad moves the pointer.
    Pointer,
    /// A stick or the D-pad gives Up, Down, Left and Right.
    Directions,
    None,
}

/// The pad controls the bindings name.
pub const PAD_CONTROLS: [&str; 15] = [
    "dpad",
    "left_stick",
    "right_stick",
    "a",
    "b",
    "x",
    "y",
    "lb",
    "rb",
    "lt",
    "rt",
    "start",
    "back",
    "ls",
    "rs",
];

#[derive(Debug, Clone, PartialEq)]
pub struct Bindings {
    pub pad: Vec<(&'static str, Bind)>,
    /// Keys by SDL scancode name; "Up" "Down" "Left" "Right" may be `Pointer` or `Directions`.
    pub keys: Vec<(String, Bind)>,
    /// The mouse moves the pointer and its buttons are Primary and Secondary.
    pub mouse: bool,
}

impl Bindings {
    fn make(pad: &[(&'static str, Bind)], keys: &[(&str, Bind)], mouse: bool) -> Bindings {
        Bindings {
            pad: PAD_CONTROLS
                .iter()
                .map(|&c| (c, pad.iter().find(|p| p.0 == c).map_or(Bind::None, |p| p.1)))
                .collect(),
            keys: keys.iter().map(|(k, b)| (k.to_string(), *b)).collect(),
            mouse,
        }
    }

    /// A pointer game (mouse-driven): sticks, D-pad and arrows move the pointer, A / Space / Return click, B / Right
    /// Ctrl is the second button, Back / Escape cancel, Start / the TV remote's Back pause.
    pub fn pointer_game() -> Bindings {
        use Action::*;
        Bindings::make(
            &[
                ("dpad", Bind::Pointer),
                ("left_stick", Bind::Pointer),
                ("right_stick", Bind::Pointer),
                ("a", Bind::Act(Primary)),
                ("b", Bind::Act(Secondary)),
                ("lb", Bind::Act(Primary)),
                ("rb", Bind::Act(Secondary)),
                ("back", Bind::Act(Cancel)),
                ("start", Bind::Act(Pause)),
            ],
            &[
                ("Up", Bind::Pointer),
                ("Down", Bind::Pointer),
                ("Left", Bind::Pointer),
                ("Right", Bind::Pointer),
                ("Space", Bind::Act(Primary)),
                ("Return", Bind::Act(Primary)),
                ("Keypad Enter", Bind::Act(Primary)),
                ("Select", Bind::Act(Primary)),
                ("Right Ctrl", Bind::Act(Secondary)),
                ("Escape", Bind::Act(Cancel)),
                ("F10", Bind::Act(Pause)),
                ("AC Back", Bind::Act(Pause)),
            ],
            true,
        )
    }

    /// A joystick game: sticks, D-pad and arrows give directions, A / Right Ctrl fire, B fire 2.
    pub fn joystick_game() -> Bindings {
        use Action::*;
        Bindings::make(
            &[
                ("dpad", Bind::Directions),
                ("left_stick", Bind::Directions),
                ("a", Bind::Act(Fire)),
                ("b", Bind::Act(Fire2)),
                ("x", Bind::Act(Start)),
                ("back", Bind::Act(Cancel)),
                ("start", Bind::Act(Pause)),
            ],
            &[
                ("Up", Bind::Directions),
                ("Down", Bind::Directions),
                ("Left", Bind::Directions),
                ("Right", Bind::Directions),
                ("Right Ctrl", Bind::Act(Fire)),
                ("Space", Bind::Act(Fire)),
                ("Return", Bind::Act(Start)),
                ("Escape", Bind::Act(Cancel)),
                ("F10", Bind::Act(Pause)),
                ("AC Back", Bind::Act(Pause)),
            ],
            false,
        )
    }
}

const DEAD: i32 = 12_000;
/// Pointer speed in logical pixels a tick: a stick fully over, and a held direction after `ACCEL_TICKS`.
const STICK_SPEED: i32 = 6;
const STEP_SLOW: i32 = 2;
const STEP_FAST: i32 = 5;
const ACCEL_TICKS: u32 = 12;

/// The player's held controls and the input they make.
#[derive(Debug, Clone)]
pub struct Mapper {
    pub bindings: Bindings,
    held: Vec<(String, Action)>,
    prev: u32,
    motion: (i32, i32),
    absolute: Option<(i32, i32)>,
    /// Stick vectors moving the pointer, by stick.
    sticks: Vec<(String, i32, i32)>,
    /// Pointer directions held by keys or D-pad (up, down, left, right) and for how many ticks.
    dirs: Vec<(String, usize)>,
    dir_ticks: u32,
    /// Actions pressed since the last tick: a press and release between two ticks (a quick tap or click) still
    /// counts as held for one tick, so it is never lost.
    latched: u32,
}

impl Mapper {
    pub fn new(bindings: Bindings) -> Mapper {
        Mapper {
            bindings,
            held: vec![],
            prev: 0,
            motion: (0, 0),
            absolute: None,
            sticks: vec![],
            dirs: vec![],
            dir_ticks: 0,
            latched: 0,
        }
    }

    fn pad_bind(&self, c: &str) -> Bind {
        self.bindings
            .pad
            .iter()
            .find(|p| p.0 == c)
            .map_or(Bind::None, |p| p.1)
    }

    fn hold(&mut self, src: &str, a: Action, down: bool) {
        self.held.retain(|h| !(h.0 == src && h.1 == a));
        if down {
            self.held.push((src.to_string(), a));
            self.latched |= a.bit();
        }
    }

    fn dir_index(name: &str) -> Option<usize> {
        match name {
            "Up" | "up" => Some(0),
            "Down" | "down" => Some(1),
            "Left" | "left" => Some(2),
            "Right" | "right" => Some(3),
            _ => None,
        }
    }

    fn direction(&mut self, src: &str, i: usize, bind: Bind, down: bool) {
        match bind {
            Bind::Pointer => {
                self.dirs.retain(|d| !(d.0 == src && d.1 == i));
                if down {
                    self.dirs.push((src.to_string(), i));
                }
            }
            Bind::Directions => {
                let a = [Action::Up, Action::Down, Action::Left, Action::Right][i];
                self.hold(src, a, down);
            }
            _ => {}
        }
    }

    /// One host event; true when it asks for the pause menu.
    pub fn handle(&mut self, e: &Host) -> bool {
        match e {
            Host::Key(name, down) => {
                let b = self
                    .bindings
                    .keys
                    .iter()
                    .find(|k| k.0 == *name)
                    .map(|k| k.1);
                match (b, Self::dir_index(name)) {
                    (Some(Bind::Act(Action::Pause)), _) => return *down,
                    (Some(Bind::Act(a)), _) => self.hold(name, a, *down),
                    (Some(b @ (Bind::Pointer | Bind::Directions)), Some(i)) => {
                        self.direction(name, i, b, *down)
                    }
                    _ => {}
                }
            }
            Host::Pad(b, down) => {
                if let Some(d) = b.strip_prefix("dpad_") {
                    if let Some(i) = Self::dir_index(d) {
                        let bind = self.pad_bind("dpad");
                        self.direction(b, i, bind, *down);
                    }
                } else {
                    match self.pad_bind(b) {
                        Bind::Act(Action::Pause) => return *down,
                        Bind::Act(a) => self.hold(b, a, *down),
                        _ => {}
                    }
                }
            }
            Host::Axis(name, x, y) => match self.pad_bind(name) {
                Bind::Pointer => {
                    self.sticks.retain(|s| s.0 != *name);
                    self.sticks.push((name.clone(), *x, *y));
                }
                Bind::Directions => {
                    for (i, on) in [*y < -DEAD, *y > DEAD, *x < -DEAD, *x > DEAD]
                        .into_iter()
                        .enumerate()
                    {
                        self.direction(name, i, Bind::Directions, on);
                    }
                }
                Bind::Act(Action::Pause) => return *x > 16_384,
                Bind::Act(a) => self.hold(name, a, *x > 16_384),
                Bind::None => {}
            },
            Host::MouseMove(dx, dy) if self.bindings.mouse => {
                self.motion.0 += dx;
                self.motion.1 += dy;
            }
            Host::MouseButton(left, down) if self.bindings.mouse => {
                let a = if *left {
                    Action::Primary
                } else {
                    Action::Secondary
                };
                self.hold("mouse", a, *down);
            }
            Host::PointerAt(x, y) => self.absolute = Some((*x, *y)),
            Host::Touch(_, Action::Pause, down) => return *down,
            Host::Touch(src, a, down) => self.hold(src, *a, *down),
            _ => {}
        }
        false
    }

    fn stick(v: i32) -> i32 {
        if v.abs() < DEAD {
            0
        } else {
            let k = (v.abs() - DEAD) as f32 / (32_767 - DEAD) as f32;
            ((STICK_SPEED as f32 * k).round() as i32).max(1) * v.signum()
        }
    }

    /// The input of the coming tick.
    pub fn tick(&mut self) -> InputState {
        let mut m = std::mem::take(&mut self.motion);
        for (_, x, y) in &self.sticks {
            m.0 += Self::stick(*x);
            m.1 += Self::stick(*y);
        }
        if self.dirs.is_empty() {
            self.dir_ticks = 0;
        } else {
            self.dir_ticks += 1;
            let step = if self.dir_ticks > ACCEL_TICKS {
                STEP_FAST
            } else {
                STEP_SLOW
            };
            let on = |i| self.dirs.iter().any(|d| d.1 == i);
            m.0 += (on(3) as i32 - on(2) as i32) * step;
            m.1 += (on(1) as i32 - on(0) as i32) * step;
        }
        let held =
            self.held.iter().fold(0, |h, x| h | x.1.bit()) | std::mem::take(&mut self.latched);
        let s = InputState {
            held,
            pressed: held & !self.prev,
            released: self.prev & !held,
            motion: m,
            absolute: self.absolute.take(),
        };
        self.prev = held;
        s
    }

    /// Lets go of everything (on pause, so nothing stays held while the game is frozen).
    pub fn release_all(&mut self) {
        self.held.clear();
        self.latched = 0;
        self.sticks.clear();
        self.dirs.clear();
        self.motion = (0, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pointer_game_moves_the_pointer_and_clicks() {
        let mut m = Mapper::new(Bindings::pointer_game());
        m.handle(&Host::MouseMove(3, -2));
        m.handle(&Host::MouseButton(true, true));
        let s = m.tick();
        assert_eq!(s.motion, (3, -2));
        assert!(s.held(Action::Primary) && s.pressed(Action::Primary));
        let s = m.tick();
        assert!(s.held(Action::Primary) && !s.pressed(Action::Primary));
        assert_eq!(s.motion, (0, 0));
        m.handle(&Host::MouseButton(true, false));
        assert!(m.tick().released(Action::Primary));
        // a click that goes down and up between two ticks is still seen for one tick
        m.handle(&Host::MouseButton(true, true));
        m.handle(&Host::MouseButton(true, false));
        let s = m.tick();
        assert!(s.held(Action::Primary) && s.pressed(Action::Primary));
        assert!(m.tick().released(Action::Primary));
        // the D-pad moves the pointer, slowly then faster
        m.handle(&Host::Pad("dpad_right".into(), true));
        assert_eq!(m.tick().motion, (STEP_SLOW, 0));
        for _ in 0..ACCEL_TICKS {
            m.tick();
        }
        assert_eq!(m.tick().motion, (STEP_FAST, 0));
        m.handle(&Host::Pad("dpad_right".into(), false));
        // a stick fully over moves it STICK_SPEED, inside the dead zone not at all
        m.handle(&Host::Axis("left_stick".into(), 32_767, 5_000));
        assert_eq!(m.tick().motion, (STICK_SPEED, 0));
        // a touch places it
        m.handle(&Host::PointerAt(100, 50));
        assert_eq!(m.tick().absolute, Some((100, 50)));
        assert!(m.handle(&Host::Pad("start".into(), true)));
        assert!(m.handle(&Host::Key("AC Back".into(), true)));
        m.handle(&Host::Key("Escape".into(), true));
        assert!(m.tick().pressed(Action::Cancel));
    }

    #[test]
    fn joystick_game_gives_directions_and_fire() {
        let mut m = Mapper::new(Bindings::joystick_game());
        m.handle(&Host::Key("Left".into(), true));
        m.handle(&Host::Pad("a".into(), true));
        m.handle(&Host::Axis("left_stick".into(), 0, 30_000));
        let s = m.tick();
        assert!(s.held(Action::Left) && s.held(Action::Fire) && s.held(Action::Down));
        assert_eq!(s.motion, (0, 0));
        m.handle(&Host::MouseMove(5, 5));
        assert_eq!(m.tick().motion, (0, 0), "no mouse in a joystick game");
        m.release_all();
        assert_eq!(m.tick().held, 0);
        assert_eq!(Action::parse("secondary"), Some(Action::Secondary));
    }
}
