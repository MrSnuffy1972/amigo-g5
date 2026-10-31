//! Save data: a small key-value store a game (and the front door) keeps between runs, written as JSON in the
//! platform's per-game storage directory (or nowhere, in headless runs and tests).

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde_json::Value;

const FILE: &str = "save.json";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Save {
    dir: Option<PathBuf>,
    values: BTreeMap<String, Value>,
}

impl Save {
    /// A store kept in memory only.
    pub fn memory() -> Save {
        Save::default()
    }

    /// The store in `dir` (created on the first write); a missing or damaged file starts empty.
    pub fn open(dir: PathBuf) -> Save {
        let values = std::fs::read(dir.join(FILE))
            .ok()
            .and_then(|b| serde_json::from_slice::<BTreeMap<String, Value>>(&b).ok())
            .unwrap_or_default();
        Save {
            dir: Some(dir),
            values,
        }
    }

    pub fn get(&self, k: &str) -> Option<&Value> {
        self.values.get(k)
    }

    pub fn get_i64(&self, k: &str) -> Option<i64> {
        self.get(k).and_then(Value::as_i64)
    }

    /// Sets a value and writes the store (a write failure leaves the value in memory and is returned).
    pub fn set(&mut self, k: &str, v: Value) -> Result<(), String> {
        self.values.insert(k.to_string(), v);
        self.flush()
    }

    pub fn flush(&self) -> Result<(), String> {
        let Some(d) = &self.dir else { return Ok(()) };
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
        let tmp = d.join(format!("{FILE}.new"));
        std::fs::write(
            &tmp,
            serde_json::to_vec_pretty(&self.values).unwrap_or_default(),
        )
        .map_err(|e| e.to_string())?;
        std::fs::rename(&tmp, d.join(FILE)).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_survive_reopening() {
        let d = std::env::temp_dir().join(format!("native-save-{}", std::process::id()));
        let mut s = Save::open(d.clone());
        assert_eq!(s.get_i64("hi"), None);
        s.set("hi", Value::from(1234)).unwrap();
        assert_eq!(Save::open(d.clone()).get_i64("hi"), Some(1234));
        std::fs::write(d.join(FILE), b"not json").unwrap();
        assert_eq!(Save::open(d.clone()).get_i64("hi"), None);
        let _ = std::fs::remove_dir_all(&d);
        let mut m = Save::memory();
        m.set("k", Value::from("v")).unwrap();
        assert_eq!(m.get("k").and_then(Value::as_str), Some("v"));
    }
}
