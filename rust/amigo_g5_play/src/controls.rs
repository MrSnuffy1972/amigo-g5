//! The game.json `controls` block and the mapping from the player's pad, keyboard and mouse to the Amiga joystick,
//! mouse and keyboard the game reads (M7, 29 Oct 2026). The mapping is data: a game without a `controls` block gets
//! the default Amiga mapping (joystick in port 2 with fire, mouse in port 1, keyboard passed through). The schema is
//! documented in docs/G5-PLAYABLE.md ("The controls block").

use amigo_g5::hardware::Input;
use serde_json::Value;

/// What a control does in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    /// The four joystick directions (the D-pad or a stick).
    Joystick,
    Fire,
    Fire2,
    /// Moves the Amiga mouse (a stick).
    Mouse,
    MouseLeft,
    MouseRight,
    /// An Amiga key, by raw key code.
    Key(u8),
    Pause,
    None,
}

impl Action {
    pub fn parse(s: &str) -> Result<Action, String> {
        Ok(match s {
            "up" => Action::Up,
            "down" => Action::Down,
            "left" => Action::Left,
            "right" => Action::Right,
            "joystick" => Action::Joystick,
            "fire" => Action::Fire,
            "fire2" => Action::Fire2,
            "mouse" => Action::Mouse,
            "mouse_left" => Action::MouseLeft,
            "mouse_right" => Action::MouseRight,
            "pause" => Action::Pause,
            "none" => Action::None,
            _ => {
                let k = s
                    .strip_prefix("key:")
                    .ok_or(format!("unknown action {s:?}"))?;
                let rk = match k.strip_prefix('$') {
                    Some(h) => u8::from_str_radix(h, 16).ok().filter(|&v| v < 0x68),
                    None => rawkey(k),
                };
                Action::Key(rk.ok_or(format!("unknown Amiga key {k:?}"))?)
            }
        })
    }
}

/// The pad's controls, by their schema names (SDL GameController buttons; Xbox layout).
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
pub struct Controls {
    /// Each pad control (from PAD_CONTROLS) and its action.
    pub pad: Vec<(&'static str, Action)>,
    /// Host keys (SDL scancode names) with their own action; they are not passed through.
    pub keys: Vec<(String, Action)>,
    /// Every other host key that has an Amiga key goes to the game as that key.
    pub passthrough: bool,
    /// The host mouse moves the Amiga mouse in port 1.
    pub mouse: bool,
    /// What the pre-boot shows for a control (M8), by pad control or key name; absent ones are named by action.
    pub labels: Vec<(String, String)>,
    /// How each label is known (`label_status`, M8): `confirmed`, `manual`, `web`, `code` or `unconfirmed`.
    pub status: Vec<(String, String)>,
    /// How the game is started (`start_hint`, M8): the text and its status.
    pub start_hint: Option<(String, String)>,
    /// The on-screen buttons from `touch.buttons` (M9 part 2): exactly four (action, label), or none given.
    pub touch_buttons: Vec<(Action, String)>,
    /// `touch.keyboard`: the game wants the pop-up keyboard.
    pub touch_keyboard: bool,
}

impl Default for Controls {
    fn default() -> Self {
        let pad = [
            ("dpad", Action::Joystick),
            ("left_stick", Action::Joystick),
            ("right_stick", Action::Mouse),
            ("a", Action::Fire),
            ("b", Action::Fire2),
            ("lb", Action::MouseLeft),
            ("rb", Action::MouseRight),
            ("start", Action::Pause),
            ("back", Action::Pause),
        ];
        let keys = [
            ("Up", Action::Up),
            ("Down", Action::Down),
            ("Left", Action::Left),
            ("Right", Action::Right),
            ("Right Ctrl", Action::Fire),
            ("Escape", Action::Pause),
            ("AC Back", Action::Pause),
        ];
        Controls {
            pad: PAD_CONTROLS
                .iter()
                .map(|&c| {
                    (
                        c,
                        pad.iter().find(|p| p.0 == c).map_or(Action::None, |p| p.1),
                    )
                })
                .collect(),
            keys: keys.iter().map(|(k, a)| (k.to_string(), *a)).collect(),
            passthrough: true,
            mouse: true,
            labels: Vec::new(),
            status: Vec::new(),
            start_hint: None,
            touch_buttons: Vec::new(),
            touch_keyboard: false,
        }
    }
}

impl Controls {
    /// The `controls` block of a game.json text; the default mapping when there is none. Fields the block leaves out
    /// keep their defaults, so a game names only what differs.
    pub fn from_game_json(text: Option<&[u8]>) -> Result<Controls, String> {
        let mut c = Controls::default();
        let Some(text) = text else { return Ok(c) };
        let v: Value = serde_json::from_slice(text).map_err(|e| format!("game.json: {e}"))?;
        let Some(b) = v.get("controls") else {
            return Ok(c);
        };
        if let Some(pad) = b.get("pad").and_then(Value::as_object) {
            for (k, a) in pad {
                let slot = c
                    .pad
                    .iter_mut()
                    .find(|p| p.0 == k)
                    .ok_or(format!("controls.pad: unknown control {k:?}"))?;
                slot.1 = Action::parse(a.as_str().unwrap_or(""))?;
            }
        }
        if let Some(keys) = b.get("keys").and_then(Value::as_object) {
            for (k, a) in keys {
                let a = Action::parse(a.as_str().unwrap_or(""))?;
                c.keys.retain(|x| x.0 != *k);
                c.keys.push((k.clone(), a));
            }
        }
        if let Some(p) = b.get("keyboard_passthrough").and_then(Value::as_bool) {
            c.passthrough = p;
        }
        if let Some(m) = b.get("mouse").and_then(Value::as_str) {
            c.mouse = m == "port1";
        }
        let strings = |k: &str| -> Vec<(String, String)> {
            b.get(k).and_then(Value::as_object).map_or(Vec::new(), |l| {
                l.iter()
                    .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                    .collect()
            })
        };
        c.labels = strings("labels");
        c.status = strings("label_status");
        if let Some(h) = b.get("start_hint") {
            let f = |k| h.get(k).and_then(Value::as_str).map(String::from);
            c.start_hint = f("text").map(|t| (t, f("status").unwrap_or_default()));
        }
        if let Some(t) = b.get("touch") {
            if let Some(bs) = t.get("buttons").and_then(Value::as_array) {
                if bs.len() != 4 {
                    return Err(
                        "controls.touch.buttons: exactly 4 buttons, each {action, label}".into(),
                    );
                }
                for x in bs {
                    let s = |k| x.get(k).and_then(Value::as_str).unwrap_or("");
                    c.touch_buttons
                        .push((Action::parse(s("action"))?, s("label").to_string()));
                }
            }
            c.touch_keyboard = t.get("keyboard").and_then(Value::as_bool) == Some(true);
        }
        Ok(c)
    }
}

/// One thing the player did, in the host's terms.
#[derive(Debug, Clone, PartialEq)]
pub enum Host {
    /// A key by SDL scancode name.
    Key(String, bool),
    /// A pad button by schema name (dpad_up, dpad_down, dpad_left and dpad_right are the D-pad).
    Pad(String, bool),
    /// A stick or trigger at a position (-32768..32767), by schema name ("left_stick", "lt").
    Axis(String, i32, i32),
    MouseMove(i32, i32),
    MouseButton(bool, bool),
    /// A finger on the touchscreen (M9 part 2): its id, what it did and where, as fractions of the window (0..1).
    Finger(i64, Phase, f32, f32),
    /// An action held or let go by an on-screen control (touch.rs), by the control's own source name.
    Touch(String, Action, bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Down,
    Move,
    Up,
}

const STICK_DEAD: i32 = 12000;
/// Amiga mouse counts a tick for a stick held fully over.
const STICK_MOUSE_SPEED: i32 = 6;

/// The player's held controls and the Amiga input they make.
#[derive(Debug, Default)]
pub struct Mapper {
    pub controls: Controls,
    /// Actions held, by the source that holds them, so two sources holding one action release it independently.
    held: Vec<(String, Action)>,
    stick_mouse: (i32, i32),
    mouse: (u8, u8),
    keys: Vec<(u8, bool)>,
}

impl Mapper {
    pub fn new(controls: Controls) -> Self {
        Mapper {
            controls,
            ..Default::default()
        }
    }

    fn pad_action(&self, control: &str) -> Action {
        self.controls
            .pad
            .iter()
            .find(|p| p.0 == control)
            .map_or(Action::None, |p| p.1)
    }

    fn hold(&mut self, src: &str, a: Action, down: bool) {
        if let Action::Key(rk) = a {
            self.keys.push((rk, down));
        }
        self.held.retain(|h| !(h.0 == src && h.1 == a));
        if down {
            self.held.push((src.to_string(), a));
        }
    }

    /// Takes one host event; true when it asks for the pause menu.
    pub fn handle(&mut self, e: &Host) -> bool {
        match e {
            Host::Key(name, down) => {
                let own = self
                    .controls
                    .keys
                    .iter()
                    .find(|k| k.0 == *name)
                    .map(|k| k.1);
                match own {
                    Some(Action::Pause) => return *down,
                    Some(a) => self.hold(name, a, *down),
                    None if self.controls.passthrough => {
                        if let Some(rk) = rawkey(name) {
                            self.keys.push((rk, *down));
                        }
                    }
                    None => {}
                }
            }
            Host::Pad(b, down) => {
                let (a, src) = match b.strip_prefix("dpad_") {
                    Some(d) if self.pad_action("dpad") == Action::Joystick => {
                        (Action::parse(d).unwrap_or(Action::None), b.as_str())
                    }
                    Some(_) => (Action::None, b.as_str()),
                    None => (self.pad_action(b), b.as_str()),
                };
                if a == Action::Pause {
                    return *down;
                }
                self.hold(src, a, *down);
            }
            Host::Axis(name, x, y) => match self.pad_action(name) {
                Action::Joystick => {
                    for (a, on) in [
                        (Action::Left, *x < -STICK_DEAD),
                        (Action::Right, *x > STICK_DEAD),
                        (Action::Up, *y < -STICK_DEAD),
                        (Action::Down, *y > STICK_DEAD),
                    ] {
                        self.hold(name, a, on);
                    }
                }
                Action::Mouse => {
                    let f = |v: i32| {
                        if v.abs() < STICK_DEAD {
                            0
                        } else {
                            v * STICK_MOUSE_SPEED / 32767
                        }
                    };
                    self.stick_mouse = (f(*x), f(*y));
                }
                // a trigger past half way is a button
                a => self.hold(name, a, *x > 16384),
            },
            Host::Touch(_, Action::Pause, down) => return *down,
            Host::Touch(src, a, down) => self.hold(src, *a, *down),
            Host::MouseMove(dx, dy) if self.controls.mouse => self.move_mouse(*dx, *dy),
            Host::MouseButton(left, down) if self.controls.mouse => {
                let a = if *left {
                    Action::MouseLeft
                } else {
                    Action::MouseRight
                };
                self.hold("mouse", a, *down);
            }
            _ => {}
        }
        false
    }

    /// The Amiga mouse counters move by the host's motion, one count a host pixel, wrapping as JOY0DAT's do.
    fn move_mouse(&mut self, dx: i32, dy: i32) {
        self.mouse.0 = self.mouse.0.wrapping_add(dx as u8);
        self.mouse.1 = self.mouse.1.wrapping_add(dy as u8);
    }

    fn is(&self, a: Action) -> bool {
        self.held.iter().any(|h| h.1 == a)
    }

    /// The input the game sees this tick (a stick held over the mouse moves it once a tick).
    pub fn tick_input(&mut self) -> Input {
        let (sx, sy) = self.stick_mouse;
        self.move_mouse(sx, sy);
        Input {
            up: self.is(Action::Up),
            down: self.is(Action::Down),
            left: self.is(Action::Left),
            right: self.is(Action::Right),
            fire: self.is(Action::Fire),
            fire2: self.is(Action::Fire2),
            mouse_joy0dat: (self.mouse.1 as u16) << 8 | self.mouse.0 as u16,
            lmb: self.is(Action::MouseLeft),
            // the right mouse button pulls POTGOR's DATLY (bit 10) low
            potgor_clear: if self.is(Action::MouseRight) {
                0x0400
            } else {
                0
            },
        }
    }

    /// Amiga key presses and releases since the last call, in order.
    pub fn take_keys(&mut self) -> Vec<(u8, bool)> {
        std::mem::take(&mut self.keys)
    }

    /// Lets go of everything (on pause, so no control stays held while the game is frozen).
    pub fn release_all(&mut self) {
        for (_, a) in std::mem::take(&mut self.held) {
            if let Action::Key(rk) = a {
                self.keys.push((rk, false));
            }
        }
        self.stick_mouse = (0, 0);
    }
}

/// The Amiga raw key at the position of a host key (SDL scancode name), for the keys an Amiga keyboard has.
pub fn rawkey(name: &str) -> Option<u8> {
    const ROWS: [(&str, u8); 4] = [
        ("1234567890-=\\", 0x01),
        ("QWERTYUIOP[]", 0x10),
        ("ASDFGHJKL;'", 0x20),
        ("ZXCVBNM,./", 0x31),
    ];
    let mut ch = name.chars();
    if let (Some(c), None) = (ch.next(), ch.next()) {
        let c = c.to_ascii_uppercase();
        if c == '`' {
            return Some(0x00);
        }
        return ROWS
            .iter()
            .find_map(|(r, b)| r.find(c).map(|i| b + i as u8));
    }
    if let Some(n) = name.strip_prefix('F').and_then(|n| n.parse::<u8>().ok()) {
        if (1..=10).contains(&n) {
            return Some(0x4F + n);
        }
    }
    if let Some(k) = name.strip_prefix("Keypad ") {
        return Some(match k {
            "0" => 0x0F,
            "1" | "2" | "3" => 0x1C + (k.as_bytes()[0] - b'0'),
            "4" | "5" | "6" => 0x29 + (k.as_bytes()[0] - b'0'),
            "7" | "8" | "9" => 0x36 + (k.as_bytes()[0] - b'0'),
            "." => 0x3C,
            "Enter" => 0x43,
            "-" => 0x4A,
            "(" => 0x5A,
            ")" => 0x5B,
            "/" => 0x5C,
            "*" => 0x5D,
            "+" => 0x5E,
            _ => return None,
        });
    }
    SPECIAL.iter().find(|k| k.0 == name).map(|k| k.1)
}

/// The host keys with an Amiga key that are not a character or a function key.
const SPECIAL: [(&str, u8); 21] = [
    ("Space", 0x40),
    ("Backspace", 0x41),
    ("Tab", 0x42),
    ("Return", 0x44),
    ("Escape", 0x45),
    ("Delete", 0x46),
    ("Up", 0x4C),
    ("Down", 0x4D),
    ("Right", 0x4E),
    ("Left", 0x4F),
    ("F11", 0x5F), // Help
    ("Insert", 0x5F),
    ("Left Shift", 0x60),
    ("Right Shift", 0x61),
    ("CapsLock", 0x62),
    ("Left Ctrl", 0x63),
    ("Right Ctrl", 0x63),
    ("Left Alt", 0x64),
    ("Right Alt", 0x65),
    ("Left GUI", 0x66),
    ("Right GUI", 0x67),
];

/// A name for an Amiga raw key, for the pre-boot's labels (M8): the host key at its place, Help for $5F.
pub fn key_name(rk: u8) -> String {
    let chars = "`1234567890-=\\QWERTYUIOP[]ASDFGHJKL;'ZXCVBNM,./";
    if rk == 0x5F {
        return "Help".into();
    }
    if let Some(c) = chars.chars().find(|c| rawkey(&c.to_string()) == Some(rk)) {
        return c.to_string();
    }
    if (0x50..=0x59).contains(&rk) {
        return format!("F{}", rk - 0x4F);
    }
    match SPECIAL.iter().find(|k| k.1 == rk) {
        Some(k) => k.0.into(),
        None => format!("${rk:02X}"),
    }
}

/// What an action does, in words, for the pre-boot's labels (M8).
pub fn describe(a: Action) -> String {
    match a {
        Action::Up => "up".into(),
        Action::Down => "down".into(),
        Action::Left => "left".into(),
        Action::Right => "right".into(),
        Action::Joystick => "joystick".into(),
        Action::Fire => "fire".into(),
        Action::Fire2 => "fire 2".into(),
        Action::Mouse => "mouse".into(),
        Action::MouseLeft => "left mouse button".into(),
        Action::MouseRight => "right mouse button".into(),
        Action::Key(rk) => format!("key {}", key_name(rk)),
        Action::Pause => "pause".into(),
        Action::None => "none".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rawkeys_follow_the_amiga_keyboard() {
        for (n, rk) in [
            ("1", 0x01),
            ("0", 0x0A),
            ("Q", 0x10),
            ("P", 0x19),
            ("A", 0x20),
            ("L", 0x28),
            ("Z", 0x31),
            ("M", 0x37),
            ("/", 0x3A),
            ("Keypad 1", 0x1D),
            ("Keypad 5", 0x2E),
            ("Keypad 9", 0x3F),
            ("F1", 0x50),
            ("F10", 0x59),
            ("Space", 0x40),
            ("Return", 0x44),
            ("Left", 0x4F),
            ("Right GUI", 0x67),
        ] {
            assert_eq!(rawkey(n), Some(rk), "{n}");
        }
        assert_eq!(rawkey("F12"), None);
        // F11 is the Help key (it used to stop at the function-key test, 29 Oct 2026)
        assert_eq!(rawkey("F11"), Some(0x5F));
        assert_eq!(rawkey("Volume Up"), None);
    }

    #[test]
    fn default_mapping_is_joystick_fire_mouse_and_passthrough() {
        let mut m = Mapper::new(Controls::default());
        m.handle(&Host::Pad("dpad_left".into(), true));
        m.handle(&Host::Key("Up".into(), true));
        m.handle(&Host::Pad("a".into(), true));
        m.handle(&Host::Key("Space".into(), true));
        m.handle(&Host::MouseMove(-3, 5));
        m.handle(&Host::MouseButton(false, true));
        let i = m.tick_input();
        assert!(i.left && i.up && i.fire && !i.right && !i.down && !i.fire2);
        assert_eq!(i.mouse_joy0dat, 0x05FD);
        assert_eq!(i.potgor_clear, 0x0400);
        // the arrow is the joystick, not a key; Space passes through as the Amiga's Space
        assert_eq!(m.take_keys(), vec![(0x40, true)]);
        assert!(m.handle(&Host::Pad("start".into(), true)));
        assert!(m.handle(&Host::Key("Escape".into(), true)));
        m.handle(&Host::Axis("left_stick".into(), 30000, 0));
        m.handle(&Host::Pad("dpad_left".into(), false));
        let i = m.tick_input();
        assert!(i.right && !i.left && i.up);
        m.release_all();
        assert!(!m.tick_input().up);
    }

    #[test]
    fn controls_block_overrides_only_what_it_names() {
        let j = br#"{"name":"x","controls":{"pad":{"x":"key:Space","b":"none"},"keys":{"Left Ctrl":"fire2"},
            "keyboard_passthrough":false,"mouse":"none","labels":{"x":"Smart bomb"}}}"#;
        let c = Controls::from_game_json(Some(j)).unwrap();
        assert!(c.pad.contains(&("x", Action::Key(0x40))));
        assert!(c.pad.contains(&("b", Action::None)));
        assert!(c.pad.contains(&("a", Action::Fire)));
        assert!(c.keys.contains(&("Left Ctrl".into(), Action::Fire2)));
        assert!(!c.passthrough && !c.mouse);
        assert_eq!(c.labels, vec![("x".into(), "Smart bomb".into())]);
        assert!(c.status.is_empty() && c.start_hint.is_none());
        let mut m = Mapper::new(c);
        m.handle(&Host::Pad("x".into(), true));
        m.handle(&Host::Pad("x".into(), false));
        m.handle(&Host::Key("Q".into(), true));
        m.handle(&Host::MouseMove(4, 4));
        assert_eq!(m.take_keys(), vec![(0x40, true), (0x40, false)]);
        assert_eq!(m.tick_input().mouse_joy0dat, 0);
        assert_eq!(Controls::from_game_json(None).unwrap(), Controls::default());
        assert!(Controls::from_game_json(Some(br#"{"controls":{"pad":{"z":"fire"}}}"#)).is_err());
        assert!(Action::parse("key:$68").is_err());
    }

    #[test]
    fn status_and_start_hint_are_read_and_keys_named() {
        let j = br#"{"controls":{"label_status":{"a":"manual","F1":"unconfirmed"},
            "start_hint":{"text":"F1 or fire: start","status":"web"}}}"#;
        let c = Controls::from_game_json(Some(j)).unwrap();
        assert!(c.status.contains(&("a".into(), "manual".into())));
        assert_eq!(
            c.start_hint,
            Some(("F1 or fire: start".into(), "web".into()))
        );
        for (rk, n) in [
            (0x40, "Space"),
            (0x50, "F1"),
            (0x10, "Q"),
            (0x5F, "Help"),
            (0x63, "Left Ctrl"),
        ] {
            assert_eq!(key_name(rk), n);
            assert_eq!(rawkey(n).unwrap_or(0x5F), rk);
        }
        assert_eq!(describe(Action::Key(0x45)), "key Escape");
    }
}
