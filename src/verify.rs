use crate::{db::Store, security::sha256_file};
use std::{error::Error, fs, path::Path};

#[derive(Debug, PartialEq, Eq)]
pub enum CheckResult { Match, Modified, NotEnrolled }

fn key(path: &Path) -> Result<String, Box<dyn Error>> {
    Ok(fs::canonicalize(path)?.to_string_lossy().into_owned())
}

pub fn enroll_file(store: &Store, path: &Path) -> Result<String, Box<dyn Error>> {
    let digest = sha256_file(path)?;
    store.save(&key(path)?, &digest)?;
    Ok(digest)
}

pub fn check_file(store: &Store, path: &Path) -> Result<CheckResult, Box<dyn Error>> {
    let expected = store.expected(&key(path)?)?;
    match expected {
        None => Ok(CheckResult::NotEnrolled),
        Some(digest) => {
            let actual = sha256_file(path)?;
            Ok(if actual == digest { CheckResult::Match } else { CheckResult::Modified })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detects_changes() {
        let dir = std::env::temp_dir().join(format!("fg_independent_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("sample.txt");
        let db = dir.join("fingerprints.db");
        std::fs::write(&file, "before").unwrap();
        let store = Store::open(&db).unwrap();
        assert_eq!(check_file(&store, &file).unwrap(), CheckResult::NotEnrolled);
        enroll_file(&store, &file).unwrap();
        assert_eq!(check_file(&store, &file).unwrap(), CheckResult::Match);
        std::fs::write(&file, "after").unwrap();
        assert_eq!(check_file(&store, &file).unwrap(), CheckResult::Modified);
        drop(store);
        std::fs::remove_dir_all(dir).unwrap();
    }
}
