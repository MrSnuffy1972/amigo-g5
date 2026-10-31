//! Scripted input for headless runs and tests, one event a line (the same lines the G5 play binary reads, plus
//! `pointer`): `TICK key down|up NAME`, `TICK pad down|up NAME`, `TICK axis NAME X Y`, `TICK mouse DX DY`,
//! `TICK button down|up left|right`, `TICK pointer X Y` (absolute, logical pixels). Ticks count from 1; an event is
//! taken just before its tick runs. Blank lines and `#` comments are skipped.

use crate::input::Host;

pub fn parse(text: &str) -> Result<Vec<(u32, Host)>, String> {
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
        let down = w.get(2).map(|s| *s == "down").unwrap_or(false);
        let e = match (w.get(1).copied(), w.len()) {
            (Some("key"), 4..) => Host::Key(w[3..].join(" "), down),
            (Some("pad"), 4) => Host::Pad(w[3].into(), down),
            (Some("axis"), 5) => Host::Axis(w[2].into(), num(w[3])?, num(w[4])?),
            (Some("mouse"), 4) => Host::MouseMove(num(w[2])?, num(w[3])?),
            (Some("button"), 4) => Host::MouseButton(w[3] == "left", down),
            (Some("pointer"), 4) => Host::PointerAt(num(w[2])?, num(w[3])?),
            _ => return Err(bad()),
        };
        out.push((tick, e));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_parse() {
        let s = "# c\n3 key down Right Ctrl\n4 pad up a\n5 axis left_stick -30000 0\n6 mouse 2 -1\n7 button down right\n8 pointer 10 20\n";
        assert_eq!(
            parse(s).unwrap(),
            vec![
                (3, Host::Key("Right Ctrl".into(), true)),
                (4, Host::Pad("a".into(), false)),
                (5, Host::Axis("left_stick".into(), -30000, 0)),
                (6, Host::MouseMove(2, -1)),
                (7, Host::MouseButton(false, true)),
                (8, Host::PointerAt(10, 20)),
            ]
        );
        assert!(parse("x key down A").is_err());
        assert!(parse("1 wiggle").is_err());
    }
}
