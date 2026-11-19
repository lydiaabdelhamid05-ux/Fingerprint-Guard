# Fingerprint Guard

**File Integrity Verification Using Rust, SHA-256 and SQLite**

Fingerprint Guard is a lightweight Rust application designed to verify file integrity using cryptographic fingerprints.

The application calculates SHA-256 hashes, stores them in a local SQLite database, and detects changes by comparing current file fingerprints against previously recorded values.

This project demonstrates fundamental concepts of file integrity monitoring, cryptographic hashing, and local database management.

## Technologies

- **Rust** — Application development
- **SHA-256** — Cryptographic file fingerprinting
- **SQLite** — Local fingerprint storage
- **rusqlite** — SQLite integration with Rust
- **Cargo** — Build system and unit testing

## Features

### SHA-256 Fingerprint Generation

Calculate a SHA-256 fingerprint from a local file.

### SQLite Fingerprint Database

Store file paths and their corresponding SHA-256 fingerprints in a local SQLite database.

### File Integrity Verification

Compare the current fingerprint of a file with its previously stored fingerprint.

The application distinguishes three verification results:

- **MATCH** — The file matches its stored fingerprint.
- **MODIFIED** — The file content differs from the stored fingerprint.
- **NOT ENROLLED** — No reference fingerprint exists for the file.

### Automated Tests

Unit tests validate SHA-256 hashing and file modification detection.

## Architecture

Fingerprint Guard implements a local file integrity verification workflow.

The project also includes technical documentation and diagrams describing its architecture.

[View Architecture Documentation](docs/architecture.md)

### Technical Diagrams

- [System Architecture](docs/diagrams/system-architecture.png)
- [Fingerprint Guard Architecture](docs/diagrams/fingerprint-guard-architecture.png)
- [Verification Workflow](docs/diagrams/verification-workflow.png)

## Project Structure

```text
Fingerprint-Guard/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── db.rs
│   ├── security.rs
│   └── verify.rs
├── docs/
│   ├── architecture.md
│   └── diagrams/
│       ├── system-architecture.png
│       ├── fingerprint-guard-architecture.png
│       └── verification-workflow.png
├── Cargo.toml
├── Cargo.lock
├── README.md
└── .gitignore
```

## Getting Started

### Requirements

- Rust and Cargo
- Linux, macOS or Windows with a compatible Rust toolchain

### Build

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

## Usage

### 1. Create a Test File

```bash
echo "Original document" > document.txt
```

### 2. Register a Fingerprint

```bash
cargo run -- enroll fingerprints.db document.txt
```

The application calculates the SHA-256 fingerprint and stores it in SQLite.

### 3. Verify File Integrity

```bash
cargo run -- check fingerprints.db document.txt
```

Expected output:

```text
MATCH: file unchanged
```

### 4. Simulate a File Modification

```bash
echo "Unauthorized modification" >> document.txt
```

Run the verification again:

```bash
cargo run -- check fingerprints.db document.txt
```

Expected output:

```text
MODIFIED: file content differs
```

### 5. Verify an Unknown File

```bash
echo "Unknown document" > unknown.txt

cargo run -- check fingerprints.db unknown.txt
```

Expected output:

```text
NOT ENROLLED: no baseline for file
```

## Database Structure

Fingerprints are stored in a local SQLite database.

```sql
CREATE TABLE fingerprints (
    path TEXT PRIMARY KEY,
    sha256 TEXT NOT NULL
);
```

Each record associates a file path with its reference SHA-256 fingerprint.

## Testing

The application includes automated tests covering:

- SHA-256 fingerprint calculation
- File modification detection

The current implementation has successfully passed both unit tests.

## Security Considerations

SHA-256 provides a cryptographic fingerprint of file contents.

Fingerprint comparison can identify modifications, but it does not determine whether a file is malicious.

The local SQLite database is not encrypted by default.

An attacker who can modify both the file and its stored reference fingerprint may bypass this basic integrity check.

This project is intended for educational and portfolio purposes rather than production endpoint protection.

## Future Improvements

Potential extensions include:

- Centralized fingerprint management using PostgreSQL
- PostgreSQL-to-SQLite synchronization
- SHA3-256 support
- Incremental fingerprint updates
- Merkle tree verification
- Parallel file processing
- JSON verification reports
- Encrypted local database storage

These features are not part of the current implementation.

## Project Status

**Functional educational prototype.**

The current version demonstrates SHA-256 fingerprint generation, SQLite storage, and local file integrity verification.
