---
bump: patch
---

### Fixed
- Fix CI/CD auto-release pipeline failing to parse pre-release semver versions (e.g., `0.1.0-beta.1`) in `Cargo.toml`
- Updated version parsing regex in `version-and-commit.rs` and `bump-version.rs` to support optional pre-release suffixes
