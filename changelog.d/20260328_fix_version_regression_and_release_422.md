---
bump: patch
---

### Fixed

- Fixed `version-and-commit.rs` not updating `Cargo.toml` when a git tag for the bumped version already exists — it now syncs `Cargo.toml` to the existing tag version and commits the change, preventing version regression (e.g. `Cargo.toml` staying at `0.1.0-beta.1` while `v0.2.0` tag existed)
- Fixed CI/CD auto-release job failing with HTTP 422 when creating a GitHub release for a version that already exists — `create-github-release.rs` now recognizes "Validation Failed" as an "already exists" condition and skips gracefully instead of erroring
- Fixed auto-release job passing stale `Cargo.toml` version to the `Create GitHub Release` step when a git tag already exists — now uses the authoritative `new_version` output from `version-and-commit.rs`, consistent with how manual-release already worked
