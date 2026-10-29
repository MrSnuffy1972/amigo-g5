//! The game pack (M7, 29 Oct 2026): a game's files and its game.json in one blob, so a Windows exe can embed it and an
//! Android APK can carry it as the asset `gamepack.bin`. Written by `tools/g5_pack.py`; the format is documented in
//! docs/G5-PLAYABLE.md ("Game pack format") and is the same on both sides:
//!
//!   "AG5PACK1"  then u32 LE entry count
//!   per entry:  u16 LE name length, the name (UTF-8, '/' separated), u32 LE data length, the data
//!   trailer:    u64 LE FNV-1a 64 of every byte before it
//!
//! Entry names: `game.json` (optional) and `files/<path>` for each game file, in directory-walk order.

use crate::game::GameTree;

pub const MAGIC: &[u8; 8] = b"AG5PACK1";

/// A pack's contents: the game's files and its game.json, when it has one.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pack {
    pub config: Option<Vec<u8>>,
    pub tree: GameTree,
}

pub fn fnv1a64(d: &[u8]) -> u64 {
    d.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| {
        (h ^ b as u64).wrapping_mul(0x0100_0000_01b3)
    })
}

impl Pack {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut ents: Vec<(String, &[u8])> = Vec::new();
        if let Some(c) = &self.config {
            ents.push(("game.json".into(), c));
        }
        for (n, d) in &self.tree.files {
            ents.push((format!("files/{n}"), d));
        }
        let mut b = MAGIC.to_vec();
        b.extend_from_slice(&(ents.len() as u32).to_le_bytes());
        for (n, d) in ents {
            b.extend_from_slice(&(n.len() as u16).to_le_bytes());
            b.extend_from_slice(n.as_bytes());
            b.extend_from_slice(&(d.len() as u32).to_le_bytes());
            b.extend_from_slice(d);
        }
        let h = fnv1a64(&b);
        b.extend_from_slice(&h.to_le_bytes());
        b
    }

    pub fn from_bytes(b: &[u8]) -> Result<Pack, String> {
        if b.len() < 20 || &b[..8] != MAGIC {
            return Err("not a game pack (no AG5PACK1 header)".into());
        }
        let (body, tail) = b.split_at(b.len() - 8);
        if fnv1a64(body).to_le_bytes() != tail {
            return Err("game pack checksum does not match (damaged or cut short)".into());
        }
        let mut at = 8;
        let mut take = |n: usize| -> Result<&[u8], String> {
            let s = body
                .get(at..at + n)
                .ok_or("game pack entry runs past its end")?;
            at += n;
            Ok(s)
        };
        let count = u32::from_le_bytes(take(4)?.try_into().unwrap());
        let mut p = Pack::default();
        for _ in 0..count {
            let nl = u16::from_le_bytes(take(2)?.try_into().unwrap()) as usize;
            let name =
                String::from_utf8(take(nl)?.to_vec()).map_err(|_| "entry name is not UTF-8")?;
            let dl = u32::from_le_bytes(take(4)?.try_into().unwrap()) as usize;
            let data = take(dl)?.to_vec();
            if name == "game.json" {
                p.config = Some(data);
            } else if let Some(rel) = name.strip_prefix("files/") {
                p.tree.files.push((rel.to_string(), data));
            } else {
                return Err(format!("unknown game pack entry {name}"));
            }
        }
        if at != body.len() {
            return Err("game pack has bytes after its last entry".into());
        }
        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_damage_is_caught() {
        let p = Pack {
            config: Some(br#"{"controls":{}}"#.to_vec()),
            tree: GameTree {
                files: vec![
                    ("Game".into(), vec![0, 0, 3, 0xF3]),
                    ("s/startup-sequence".into(), b"Game\n".to_vec()),
                ],
            },
        };
        let b = p.to_bytes();
        assert_eq!(Pack::from_bytes(&b).unwrap(), p);
        let mut bad = b.clone();
        bad[20] ^= 1;
        assert!(Pack::from_bytes(&bad).is_err());
        assert!(Pack::from_bytes(&b[..b.len() - 1]).is_err());
    }
}
