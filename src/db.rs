use rusqlite::{params, Connection, Result};
use std::path::Path;

pub struct Store { conn: Connection }
impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("CREATE TABLE IF NOT EXISTS fingerprints (path TEXT PRIMARY KEY, sha256 TEXT NOT NULL);")?;
        Ok(Self { conn })
    }
    pub fn save(&self, path: &str, sha256: &str) -> Result<()> {
        self.conn.execute("INSERT INTO fingerprints(path, sha256) VALUES (?1, ?2) ON CONFLICT(path) DO UPDATE SET sha256=excluded.sha256", params![path, sha256])?;
        Ok(())
    }
    pub fn expected(&self, path: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT sha256 FROM fingerprints WHERE path = ?1")?;
        let mut rows = stmt.query([path])?;
        Ok(rows.next()?.map(|row| row.get(0)).transpose()?)
    }
}
