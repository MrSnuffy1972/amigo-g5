//! The asset pack: format `native-asset-pack/1` (it replaces the `0.1-proposed` draft; see docs/G5-NATIVE.md "Asset
//! packs"). A pack is a manifest (JSON) naming each asset with its kind and metadata, plus the asset files. It exists
//! in two forms with the same manifest: a directory (manifest.json beside the files, for tools and development) and a
//! single binary file that a program embeds or an APK carries as an asset:
//!
//! ```text
//! "NAP1"  u32 LE version (1)  u32 LE manifest length  manifest JSON (UTF-8)  file bytes...  u64 LE FNV-1a of all before
//! ```
//!
//! In the binary form every asset entry also has `offset` and `len` (into the file bytes after the manifest). Asset
//! kinds: `palette` (JSON `colours` as #RRGGBB), `image` (raw 8-bit palette indices, `w` x `h` per frame, `frames`
//! side by side), `font` (an image of glyph cells, 1 = ink, with `glyph_w`, `glyph_h`, `first`, `count`, optional
//! `fold_lower`, `alias`, `blank`), `sample` (signed 8-bit PCM, `format` "s8"), `music` (`format` "protracker-mod"),
//! `data` (any JSON).

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use serde_json::{json, Value};

use crate::audio::Sample;
use crate::gfx::{Bitmap, Font, Palette, Sheet};

pub const FORMAT: &str = "native-asset-pack/1";
const MAGIC: &[u8; 4] = b"NAP1";

pub fn fnv1a64(d: &[u8]) -> u64 {
    d.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| {
        (h ^ b as u64).wrapping_mul(0x0100_0000_01b3)
    })
}

#[derive(Debug, Clone)]
pub struct Pack {
    pub manifest: Value,
    files: BTreeMap<String, Vec<u8>>,
}

impl Pack {
    /// A pack directory: manifest.json and the files its assets name by `path`.
    pub fn from_dir(dir: &Path) -> Result<Pack, String> {
        let m = std::fs::read(dir.join("manifest.json"))
            .map_err(|e| format!("{}: {e}", dir.display()))?;
        let manifest: Value =
            serde_json::from_slice(&m).map_err(|e| format!("manifest.json: {e}"))?;
        check_format(&manifest)?;
        let mut files = BTreeMap::new();
        for a in assets(&manifest) {
            let name = field(a, "name")?;
            let path = field(a, "path")?;
            let d = std::fs::read(dir.join(path)).map_err(|e| format!("{path}: {e}"))?;
            files.insert(name.to_string(), d);
        }
        Ok(Pack { manifest, files })
    }

    /// The binary form.
    pub fn from_bytes(b: &[u8]) -> Result<Pack, String> {
        if b.len() < 20 || &b[..4] != MAGIC {
            return Err("not an asset pack (no NAP1 header)".into());
        }
        let (body, tail) = b.split_at(b.len() - 8);
        if fnv1a64(body).to_le_bytes() != tail {
            return Err("asset pack checksum does not match (damaged or cut short)".into());
        }
        let ver = u32::from_le_bytes(body[4..8].try_into().unwrap());
        if ver != 1 {
            return Err(format!("asset pack version {ver}, this engine reads 1"));
        }
        let ml = u32::from_le_bytes(body[8..12].try_into().unwrap()) as usize;
        let mj = body
            .get(12..12 + ml)
            .ok_or("asset pack manifest cut short")?;
        let manifest: Value =
            serde_json::from_slice(mj).map_err(|e| format!("pack manifest: {e}"))?;
        check_format(&manifest)?;
        let data = &body[12 + ml..];
        let mut files = BTreeMap::new();
        for a in assets(&manifest) {
            let name = field(a, "name")?;
            let o = a["offset"].as_u64().ok_or(format!("{name}: no offset"))? as usize;
            let l = a["len"].as_u64().ok_or(format!("{name}: no len"))? as usize;
            let d = data
                .get(o..o + l)
                .ok_or(format!("{name}: outside the pack"))?;
            files.insert(name.to_string(), d.to_vec());
        }
        Ok(Pack { manifest, files })
    }

    /// The binary form of this pack.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut m = self.manifest.clone();
        let mut blob = Vec::new();
        if let Some(list) = m["assets"].as_array_mut() {
            for a in list {
                let name = a["name"].as_str().unwrap_or("").to_string();
                let d = self.files.get(&name).map(Vec::as_slice).unwrap_or(&[]);
                a["offset"] = json!(blob.len());
                a["len"] = json!(d.len());
                blob.extend_from_slice(d);
            }
        }
        let mj = serde_json::to_vec(&m).unwrap_or_default();
        let mut out = Vec::with_capacity(20 + mj.len() + blob.len());
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&1u32.to_le_bytes());
        out.extend_from_slice(&(mj.len() as u32).to_le_bytes());
        out.extend_from_slice(&mj);
        out.extend_from_slice(&blob);
        let h = fnv1a64(&out);
        out.extend_from_slice(&h.to_le_bytes());
        out
    }

    /// A pack made in code (tests, tools).
    pub fn new(manifest: Value, files: BTreeMap<String, Vec<u8>>) -> Result<Pack, String> {
        check_format(&manifest)?;
        Ok(Pack { manifest, files })
    }

    pub fn title(&self) -> &str {
        self.manifest["title"].as_str().unwrap_or("")
    }

    /// The manifest entry of asset `name`, checked to be of `kind`.
    pub fn entry(&self, name: &str, kind: &str) -> Result<&Value, String> {
        let a = assets(&self.manifest)
            .find(|a| a["name"] == name)
            .ok_or(format!("asset pack has no {name:?}"))?;
        if a["kind"] != kind {
            return Err(format!("{name}: is a {}, not a {kind}", a["kind"]));
        }
        Ok(a)
    }

    pub fn bytes(&self, name: &str) -> Result<&[u8], String> {
        self.files
            .get(name)
            .map(Vec::as_slice)
            .ok_or(format!("asset pack has no file for {name:?}"))
    }

    pub fn palette(&self, name: &str) -> Result<Palette, String> {
        self.entry(name, "palette")?;
        let v: Value =
            serde_json::from_slice(self.bytes(name)?).map_err(|e| format!("{name}: {e}"))?;
        let cols: Vec<&str> = v["colours"]
            .as_array()
            .ok_or(format!("{name}: no colours"))?
            .iter()
            .filter_map(Value::as_str)
            .collect();
        Palette::from_hex(&cols)
    }

    /// An image as a sheet of its frames.
    pub fn sheet(&self, name: &str) -> Result<Sheet, String> {
        let a = self.entry(name, "image")?;
        let (w, h) = (num(a, "w")?, num(a, "h")?);
        let n = a["frames"].as_u64().unwrap_or(1) as usize;
        let b = Bitmap::from_indices(w * n, h, self.bytes(name)?.to_vec())
            .map_err(|e| format!("{name}: {e}"))?;
        Sheet::new(b, w, n)
    }

    /// A single-frame image.
    pub fn bitmap(&self, name: &str) -> Result<Bitmap, String> {
        Ok(self.sheet(name)?.image)
    }

    pub fn font(&self, name: &str) -> Result<Font, String> {
        let a = self.entry(name, "font")?;
        let (gw, gh, n) = (num(a, "glyph_w")?, num(a, "glyph_h")?, num(a, "count")?);
        let b = Bitmap::from_indices(gw * n, gh, self.bytes(name)?.to_vec())
            .map_err(|e| format!("{name}: {e}"))?;
        let ch = |v: &Value| v.as_str().and_then(|s| s.chars().next());
        Ok(Font {
            glyphs: Sheet::new(b, gw, n)?,
            first: num(a, "first")? as u32,
            fold_lower: a["fold_lower"].as_bool().unwrap_or(false),
            alias: a["alias"]
                .as_object()
                .map(|o| {
                    o.iter()
                        .filter_map(|(k, v)| Some((k.chars().next()?, ch(v)?)))
                        .collect()
                })
                .unwrap_or_default(),
            blank: a["blank"]
                .as_str()
                .map(|s| s.chars().collect())
                .unwrap_or_default(),
        })
    }

    /// A sample at `rate` (the caller knows the rate a game plays it at).
    pub fn sample(&self, name: &str, rate: f32) -> Result<Sample, String> {
        let a = self.entry(name, "sample")?;
        if a["format"] != "s8" {
            return Err(format!("{name}: sample format {}", a["format"]));
        }
        Ok(Sample {
            data: Arc::new(self.bytes(name)?.iter().map(|&b| b as i8).collect()),
            rate,
        })
    }

    pub fn music(&self, name: &str) -> Result<&[u8], String> {
        self.entry(name, "music")?;
        self.bytes(name)
    }

    pub fn data(&self, name: &str) -> Result<Value, String> {
        self.entry(name, "data")?;
        serde_json::from_slice(self.bytes(name)?).map_err(|e| format!("{name}: {e}"))
    }
}

fn check_format(m: &Value) -> Result<(), String> {
    match m["format"].as_str() {
        Some(FORMAT) => {}
        other => {
            return Err(format!(
                "asset pack format {other:?}, this engine reads {FORMAT}"
            ))
        }
    }
    let mut seen = std::collections::BTreeSet::new();
    for a in assets(m) {
        if !seen.insert(field(a, "name")?) {
            return Err(format!("asset pack names {:?} twice", a["name"]));
        }
    }
    Ok(())
}

fn assets(m: &Value) -> impl Iterator<Item = &Value> {
    m["assets"].as_array().into_iter().flatten()
}

fn field<'a>(a: &'a Value, k: &str) -> Result<&'a str, String> {
    a[k].as_str().ok_or(format!("asset entry without {k}: {a}"))
}

fn num(a: &Value, k: &str) -> Result<usize, String> {
    a[k].as_u64()
        .map(|v| v as usize)
        .ok_or(format!("{}: no {k}", a["name"]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic() -> Pack {
        let m = json!({"format": FORMAT, "title": "Synthetic", "assets": [
            {"kind": "palette", "name": "pal", "path": "p.json"},
            {"kind": "image", "name": "img", "path": "i.idx8", "w": 2, "h": 2, "frames": 2},
            {"kind": "font", "name": "fnt", "path": "f.idx8", "glyph_w": 1, "glyph_h": 1, "first": 65, "count": 2,
             "fold_lower": true, "alias": {".": "B"}, "blank": " "},
            {"kind": "sample", "name": "snd", "path": "s.s8", "format": "s8"},
            {"kind": "data", "name": "dat", "path": "d.json"}]});
        let mut f = BTreeMap::new();
        f.insert(
            "pal".into(),
            br##"{"colours":["#000000","#FF8000"]}"##.to_vec(),
        );
        f.insert("img".into(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
        f.insert("fnt".into(), vec![1, 0]);
        f.insert("snd".into(), vec![0x7F, 0x80]);
        f.insert("dat".into(), br#"{"n":[1,2]}"#.to_vec());
        Pack::new(m, f).unwrap()
    }

    #[test]
    fn binary_form_round_trips_and_detects_damage() {
        let p = synthetic();
        let b = p.to_bytes();
        let q = Pack::from_bytes(&b).unwrap();
        assert_eq!(q.title(), "Synthetic");
        assert_eq!(q.palette("pal").unwrap().colours[1], 0xFFFF_8000);
        let s = q.sheet("img").unwrap();
        assert_eq!((s.frames, s.frame_w, s.image.get(2, 1)), (2, 2, 7));
        let f = q.font("fnt").unwrap();
        assert_eq!(
            (f.glyph('a'), f.glyph('.'), f.glyph(' ')),
            (Some(0), Some(1), None)
        );
        assert_eq!(*q.sample("snd", 8000.0).unwrap().data, vec![127, -128]);
        assert_eq!(q.data("dat").unwrap()["n"][1], 2);
        assert!(q.sheet("pal").is_err());
        let mut bad = b.clone();
        bad[20] ^= 1;
        assert!(Pack::from_bytes(&bad).is_err());
        assert!(Pack::from_bytes(b"NAP1").is_err());
        let dup = json!({"format": FORMAT, "assets": [{"kind": "data", "name": "a"}, {"kind": "image", "name": "a"}]});
        assert!(Pack::new(dup, BTreeMap::new()).is_err());
    }

    #[test]
    fn directory_form_reads_the_same() {
        let d = std::env::temp_dir().join(format!("nap-test-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        let p = synthetic();
        std::fs::write(
            d.join("manifest.json"),
            serde_json::to_vec(&p.manifest).unwrap(),
        )
        .unwrap();
        for (n, path) in [
            ("pal", "p.json"),
            ("img", "i.idx8"),
            ("fnt", "f.idx8"),
            ("snd", "s.s8"),
            ("dat", "d.json"),
        ] {
            std::fs::write(d.join(path), p.bytes(n).unwrap()).unwrap();
        }
        let q = Pack::from_dir(&d).unwrap();
        assert_eq!(q.to_bytes(), p.to_bytes());
        let _ = std::fs::remove_dir_all(&d);
    }
}
