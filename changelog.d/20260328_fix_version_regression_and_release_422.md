---
bump: patch
---

### Fixed

- Fixed CI/CD auto-release job failing with HTTP 422 when creating GitHub release for a version that already exists — `create-github-release.rs` now recognizes "Validation Failed" as an "already exists" condition and skips gracefully instead of erroring
- Fixed auto-release job passing stale Cargo.toml version to `Create GitHub Release` step when git tag already exists — now uses the authoritative `new_version` output from `version-and-commit.rs` (consistent with how manual-release already worked)
- Restored `Cargo.toml` version to `0.2.0` to match the already-released `v0.2.0` git tag (version had regressed to `0.1.0-beta.1`)
