use fingerprint_guard::{verify, verify_files, VerifyResult};
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    // SÉCURITÉ : le secret est lu depuis l'environnement, jamais codé en dur.
    // Lancement : FINGERPRINT_GUARD_SECRET=mon_secret cargo run
    let secret = match std::env::var("FINGERPRINT_GUARD_SECRET") {
        Ok(s) => s,
        Err(_) => {
            eprintln!("[ERREUR] FINGERPRINT_GUARD_SECRET n'est pas définie.");
            eprintln!("  → Lancement : FINGERPRINT_GUARD_SECRET=<secret> cargo run");
            std::process::exit(1);
        }
    };

    // Chemin de la base (DB_PATH optionnel, défaut : fingerprints.db)
    let db_path = std::env::var("DB_PATH")
        .unwrap_or_else(|_| "fingerprints.db".to_string());

    // ── Exemple 1 : vérification d'une empreinte seule ───────────────────────
    let hash = "abc123";
    match verify(hash, &secret, &db_path) {
        VerifyResult::Found         => println!("[OK] Empreinte '{}' trouvée.", hash),
        VerifyResult::NotFound      => println!("[--] Empreinte '{}' inconnue.", hash),
        VerifyResult::Unauthorized  => eprintln!("[!!] Accès refusé : secret invalide."),
        VerifyResult::DatabaseError(e) => eprintln!("[!!] Erreur base de données : {}", e),
        VerifyResult::IoError(e)    => eprintln!("[!!] Erreur I/O : {}", e),
    }

    // ── Exemple 2 : traitement parallèle avec benchmark ──────────────────────
    let fichiers: Vec<PathBuf> = vec![
        PathBuf::from("sample_report.pdf"),
        PathBuf::from("sample_scan.dcm"),
        PathBuf::from("sample_image.png"),    // filtré : extension non supportée
        PathBuf::from("sample_config.txt"),
        PathBuf::from("sample_products.csv"), // filtré : extension non supportée
        PathBuf::from("sample_document.docx"),
        PathBuf::from("sample_data.json"),
        PathBuf::from("sample_records.xml")

    ];

    println!("\nTraitement parallèle de {} fichier(s)...", fichiers.len());
    let debut = Instant::now();
    let resultats = verify_files(&fichiers, &secret, &db_path);
    let duree = debut.elapsed();

    println!(
        "{} fichier(s) traité(s) (après filtrage) en {:?}",
        resultats.len(),
        duree
    );
    for (chemin, res) in &resultats {
        println!("  {:?} → {:?}", chemin, res);
    }
}