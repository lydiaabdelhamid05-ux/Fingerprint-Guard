use rusqlite::{Connection, OpenFlags, Result};
use std::env;

pub struct Database {
    conn: Connection,
}

impl Database {
    // Ouvre la base SQLite au chemin fourni.
    // SÉCURITÉ : la clé de chiffrement (SQLCipher) est lue depuis la variable
    // d'environnement `DB_ENCRYPTION_KEY` si elle est définie. Requiert SQLCipher.
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open_with_flags(
            db_path, 
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?;

        // SÉCURITÉ : clé lue depuis l'environnement, jamais codée en dur.
        // Si DB_ENCRYPTION_KEY n'est pas définie, pas de chiffrement (mode dev/test).
        if let Ok(key) = env::var("DB_ENCRYPTION_KEY") {
            conn.pragma_update(None, "key", key.as_str())?;
        }

        // Création de la table si elle n'existe pas encore (première exécution)
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS fingerprints (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                hash TEXT    NOT NULL UNIQUE
            );"
        )?;

        Ok(Self { conn })
    }

    pub fn find_hash(&self, hash: &str) -> Result<bool> {
        let mut stmt = self.conn.prepare(
            "SELECT EXISTS(SELECT 1 FROM fingerprints WHERE hash = ?1)",
        )?;
        let exists: bool = stmt.query_row([hash], |row| row.get(0))?;
        Ok(exists)
    }
}