// point d’entrée de la bibliothèque

pub mod db;       // gestion de la base SQLite
pub mod security; // validation du secret (anti timing-attack)
pub mod verify;   // logique principale + traitement parallèle

// Exports publics : fonction unique, traitement parallèle, et type de résultat
pub use verify::{verify, verify_files, VerifyResult};