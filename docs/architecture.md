# Architecture overview

This diagram is a **conceptual overview**, not evidence of an implemented synchronization service. The published code covers only the local Rust library and CLI.

```mermaid
flowchart LR
  C[Central signature repository\nConceptual / out of scope] -. Future synchronization .-> L[(Local SQLite fingerprint store)]
  F[Local files] --> H[Streaming SHA-256 hashing]
  H --> V[Fingerprint verification]
  S[Environment secret] --> A[Constant-time access check]
  A --> V
  L --> V
  V --> R[FOUND / NOT_FOUND / UNAUTHORIZED / errors]
```

## Implemented scope
- Local SHA-256 hashing of supported files, in 8 KiB chunks.
- Rayon parallel processing of a list of paths.
- Access check against an environment variable before database lookup.
- SQLite fingerprint existence checks with explicit database and I/O error variants.

## Outside the published implementation
- PostgreSQL synchronization, message transport, Merkle tree generation, per-device salting, and cloud infrastructure.
- Encrypted SQLite requires an actual SQLCipher-enabled SQLite build; the included Cargo manifest does not configure SQLCipher.
