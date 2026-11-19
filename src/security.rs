// SÉCURITÉ : comparaison à temps constant pour prévenir les timing attacks
use subtle::ConstantTimeEq;
use std::env;

pub struct SecurityManager;

impl SecurityManager {
    /// Valide le secret fourni contre la variable d'environnement `FINGERPRINT_GUARD_SECRET`
    pub fn validate_secret(secret: &str) -> bool {
        match env::var("FINGERPRINT_GUARD_SECRET") {
            Ok(expected) => {
                // Comparaison en temps constant : indépendante du contenu des chaînes
                bool::from(expected.as_bytes().ct_eq(secret.as_bytes()))
            }
            // Variable non définie → accès refusé systématiquement
            Err(_) => false,
        }
    }
}
