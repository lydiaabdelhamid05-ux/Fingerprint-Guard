use crate::db::Database;
use crate::security::SecurityManager;
use rayon::prelude::*;      // Rayon pour le traitement parallèle multi-thread
use sha2::{Digest, Sha256};     // SHA-256 en streaming (lecture par blocs, sans tout charger en RAM)
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

// Extensions de fichiers supportées pour la vérification d'empreintes.
const ALLOWED_EXTENSIONS: &[&str] = &["pdf", "docx", "txt", "dcm", "csv", "json", "xml"];

// Résultat d'une vérification d'empreinte.

// de les masquer silencieusement en `NotFound` comme dans la version d'origine.
#[derive(Debug)]
pub enum VerifyResult {
    Found,
    NotFound,
    Unauthorized,
    DatabaseError(String), // erreur DB propagée
    IoError(String),       // erreur de lecture de fichier (séparée des erreurs DB)
}

// Vérifie si une empreinte (hash SHA-256 en hexadécimal) est présente dans la base
pub fn verify(hash: &str, secret: &str, db_path: &str) -> VerifyResult {
    // 1. Vérification du secret (obligatoire)
    if !SecurityManager::validate_secret(secret) {
        return VerifyResult::Unauthorized;
    }

    // 2. Accès DB uniquement si autorisé
    let db = match Database::new(db_path) {
        Ok(db) => db,
        Err(e) => return VerifyResult::DatabaseError(e.to_string()), // CORRECTIF : propagation
    };

    // 3. Recherche d'empreinte
    match db.find_hash(hash) {
        Ok(true)  => VerifyResult::Found,
        Ok(false) => VerifyResult::NotFound,
        Err(e)    => VerifyResult::DatabaseError(e.to_string()), 
    }
}

// Calcule le hash SHA-256 d'un fichier par lecture en streaming.
// OPTIMISATION : lecture par blocs de 8 Ko → seuls 8 Ko sont en RAM à la fois,
// quel que soit la taille du fichier (utile pour les .dcm ou .pdf volumineux).
fn hash_file(path: &Path) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192]; // blocs de 8 Ko

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(hex::encode(hasher.finalize()))
}

// Traite plusieurs fichiers en parallèle et vérifie leurs empreintes.

// OPTIMISATION : Rayon distribue le travail sur tous les cœurs CPU disponibles.
// Les fichiers dont l'extension n'est pas supportée sont filtrés avant traitement.
// Seuls les fichiers passant le filtre apparaissent dans le vecteur de résultats.

pub fn verify_files(
    paths: &[PathBuf],
    secret: &str,
    db_path: &str,
) -> Vec<(PathBuf, VerifyResult)> {
    paths
        .par_iter()
        .filter(|path| {
            // OPTIMISATION : filtrage par extension avant tout I/O sur le fichier
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ALLOWED_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
                .unwrap_or(false)
        })
        .map(|path| {
            let result = match hash_file(path) {
                Ok(hash) => verify(&hash, secret, db_path),
                Err(e)   => VerifyResult::IoError(e.to_string()),
            };
            (path.clone(), result)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    // Vérifie que seules les extensions autorisées sont acceptées.
    // Permet d’éviter de traiter des fichiers inutiles et d’optimiser les performances.
    #[test]
    fn test_filtrage_extensions() {
        let chemins = vec![
            PathBuf::from("rapport.pdf"),   
            PathBuf::from("image.png"),     
            PathBuf::from("scan.dcm"),       
            PathBuf::from("notes.txt"),     
            PathBuf::from("doc.docx"), 
            PathBuf::from("products.csv"), 
            PathBuf::from("user_data.json"),
            PathBuf::from("employees.xml")     
        ];

        let acceptes: Vec<_> = chemins
            .iter()
            .filter(|p| {
                p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| ALLOWED_EXTENSIONS.contains(&e.to_lowercase().as_str()))
                    .unwrap_or(false)
            })
            .collect();

        assert_eq!(acceptes.len(), 7); // pdf, dcm, txt, docx, csv, json, xml
    }

    // Vérifie que le hachage SHA-256 en streaming est idempotent et de la bonne longueur.
    #[test]
    fn test_hachage_streaming_coherent() {
        let chemin = PathBuf::from("/tmp/fingerprint_guard_test.txt");
        {
            let mut f = File::create(&chemin).unwrap();
            f.write_all(b"contenu de test deterministe").unwrap();
        }

        let hash1 = hash_file(&chemin).unwrap();
        let hash2 = hash_file(&chemin).unwrap();

        assert_eq!(hash1, hash2);    // idempotent : même fichier → même hash
        assert_eq!(hash1.len(), 64); // SHA-256 = 32 octets = 64 caractères hex

        std::fs::remove_file(&chemin).unwrap();
    }
}