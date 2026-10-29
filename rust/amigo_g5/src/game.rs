//! A game's files as one tree in memory, and the program the kit would boot from them (M7, 29 Oct 2026). Moved here
//! from the boot binary so that `amigo-g5-boot` (a directory) and `amigo-g5-play` (a directory or a game pack) pick
//! and boot the same program the same way; the rules are unchanged from M6 (oshle.py `startup_args`).

use std::fs;
use std::path::Path;

use crate::boot::{cold_boot, Booted};
use crate::os::GameFiles;

/// The game's files by relative path ('/' separated), in the order a directory walk sorted by name gives (each
/// directory's entries by file name, a subdirectory's files where its name sorts). The OS services see this order.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameTree {
    pub files: Vec<(String, Vec<u8>)>,
}

/// The program chosen to boot: its path in the tree, its CLI argument string (ending in a newline), and a warning
/// for the log when no startup-sequence line names it.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub rel: String,
    pub args: Vec<u8>,
    pub warning: Option<String>,
}

impl GameTree {
    /// Every file under `dir`.
    pub fn from_dir(dir: &Path) -> std::io::Result<Self> {
        let mut t = GameTree::default();
        walk(dir, "", &mut t.files)?;
        Ok(t)
    }

    pub fn get(&self, rel: &str) -> Option<&[u8]> {
        self.files
            .iter()
            .find(|(n, _)| n == rel)
            .map(|(_, d)| d.as_slice())
    }

    /// s/startup-sequence, in the spellings the kit tries, in its order.
    fn startup(&self) -> Option<&[u8]> {
        ["s", "S"]
            .iter()
            .flat_map(|d| {
                ["startup-sequence", "Startup-Sequence", "STARTUP-SEQUENCE"]
                    .iter()
                    .map(move |f| format!("{d}/{f}"))
            })
            .find_map(|p| self.get(&p))
    }

    /// The program: `program` when given (the kit's G0 choice), else the first command of s/startup-sequence that
    /// names a hunk executable at the top of the tree.
    pub fn program(&self, program: Option<&str>) -> Result<Program, String> {
        match program {
            Some(rel) => self.chosen_program(rel.trim_start_matches("./")),
            None => self.startup_program(),
        }
    }

    fn startup_program(&self) -> Result<Program, String> {
        let text = self
            .startup()
            .ok_or("no s/startup-sequence in the game's files")?;
        for line in text.split(|&b| b == b'\n') {
            let line = String::from_utf8_lossy(line);
            let line = line.split(';').next().unwrap_or("").trim();
            let mut parts = line.splitn(2, char::is_whitespace);
            let Some(cmd) = parts.next().filter(|c| !c.is_empty()) else {
                continue;
            };
            let name = cmd.rsplit([':', '/']).next().unwrap_or(cmd);
            let top = self
                .files
                .iter()
                .find(|(n, _)| !n.contains('/') && n.eq_ignore_ascii_case(name));
            if let Some((f, data)) = top {
                if is_hunk_exe(data) {
                    let mut args = parts.next().unwrap_or("").trim().as_bytes().to_vec();
                    args.push(b'\n');
                    return Ok(Program {
                        rel: f.clone(),
                        args,
                        warning: None,
                    });
                }
            }
        }
        Err("s/startup-sequence names no hunk executable at the top of the game's files".into())
    }

    /// The startup-sequence's lines, comments cut, split into words (oshle.py `startup_args`).
    fn startup_lines(&self) -> Vec<Vec<String>> {
        let Some(text) = self.startup() else {
            return Vec::new();
        };
        String::from_utf8_lossy(text)
            .lines()
            .map(|l| {
                l.split(';')
                    .next()
                    .unwrap_or("")
                    .split_whitespace()
                    .map(String::from)
                    .collect()
            })
            .collect()
    }

    /// The words after the command of the first startup-sequence line whose command's last path part is the
    /// program's name, plus a newline (oshle.py `startup_args`).
    fn chosen_program(&self, rel: &str) -> Result<Program, String> {
        if self.get(rel).is_none() {
            return Err(format!("--program {rel}: no such file in the game's files"));
        }
        let base = rel.rsplit('/').next().unwrap_or(rel).to_lowercase();
        for w in self.startup_lines() {
            if let Some(cmd) = w.first() {
                if cmd.rsplit([':', '/']).next().unwrap_or(cmd).to_lowercase() == base {
                    let mut args = w[1..].join(" ").into_bytes();
                    args.push(b'\n');
                    return Ok(Program {
                        rel: rel.to_string(),
                        args,
                        warning: None,
                    });
                }
            }
        }
        Ok(Program {
            rel: rel.to_string(),
            args: b"\n".to_vec(),
            warning: Some(format!("no startup-sequence line runs {rel} (the kit starts it from Workbench, which the port does not serve yet): started from the CLI with an empty line")),
        })
    }

    /// Cold-boots `p` with every file of the tree served by its file name (the OS services look a name up by its
    /// last path part).
    pub fn boot(&self, p: &Program) -> Result<Booted, String> {
        let mut files = GameFiles::default();
        for (rel, d) in &self.files {
            files.add(rel.rsplit('/').next().unwrap_or(rel), d.clone());
        }
        let exe = self.get(&p.rel).ok_or("program file vanished")?;
        cold_boot(exe, &p.args, files).map_err(|e| format!("cold boot failed: {e:?}"))
    }
}

fn walk(dir: &Path, prefix: &str, out: &mut Vec<(String, Vec<u8>)>) -> std::io::Result<()> {
    let mut ents: Vec<_> = fs::read_dir(dir)?.collect::<Result<_, _>>()?;
    ents.sort_by_key(|e| e.file_name());
    for e in ents {
        let p = e.path();
        let rel = format!("{prefix}{}", e.file_name().to_string_lossy());
        if p.is_dir() {
            walk(&p, &format!("{rel}/"), out)?;
        } else {
            out.push((rel, fs::read(&p)?));
        }
    }
    Ok(())
}

pub fn is_hunk_exe(d: &[u8]) -> bool {
    d.len() >= 4 && d[..4] == [0, 0, 3, 0xF3]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree(files: &[(&str, &[u8])]) -> GameTree {
        GameTree {
            files: files
                .iter()
                .map(|(n, d)| (n.to_string(), d.to_vec()))
                .collect(),
        }
    }

    const EXE: &[u8] = &[0, 0, 3, 0xF3, 0, 0, 0, 0];

    #[test]
    fn startup_sequence_names_the_program_and_its_arguments() {
        let t = tree(&[
            ("Game", EXE),
            ("notes", b"text"),
            (
                "s/startup-sequence",
                b"; boot\nnotes x\ndf0:Game  -fast 2 ; go\n",
            ),
        ]);
        let p = t.program(None).unwrap();
        assert_eq!(p.rel, "Game");
        assert_eq!(p.args, b"-fast 2\n");
        assert!(p.warning.is_none());
    }

    #[test]
    fn chosen_program_takes_its_line_or_an_empty_one() {
        let t = tree(&[
            ("c/run", EXE),
            ("prog", EXE),
            ("s/Startup-Sequence", b"C:Prog a  b\n"),
        ]);
        let p = t.program(Some("./prog")).unwrap();
        assert_eq!((p.rel.as_str(), p.args.as_slice()), ("prog", &b"a b\n"[..]));
        let p = t.program(Some("c/run")).unwrap();
        assert_eq!(p.args, b"\n");
        assert!(p.warning.is_some());
        assert!(t.program(Some("nope")).is_err());
    }
}
