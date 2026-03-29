---
bump: patch
---

### Fixed
- Release pipeline now always publishes a version greater than what exists on crates.io
- `publish-crate.rs` now fails (exit 1) when version already exists instead of silently accepting
- `version-and-commit.rs` now queries crates.io for the max published version and ensures the new version exceeds it
- `check-release-needed.rs` now outputs `max_published_version` for diagnostic purposes

### Added
- Case study documentation for issue #22 analyzing the false-positive "already exists" bug
- Experiment script testing version bumping logic against published versions
