---
bump: minor
---

### Changed
- Migrated from nightly Rust (nightly-2022-08-22) to stable Rust toolchain
- Updated `platform-num` dependency from 0.1.0-aplha.1 to 0.5.0
- Replaced `funty` dependency with `num-traits` for stable Rust compatibility
- Updated edition from 2018 to 2021 with rust-version 1.70
- Replaced Node.js (.mjs) CI/CD scripts with Rust (.rs) scripts using rust-script
- Added clippy pedantic/nursery lints and release profile optimizations

### Added
- Lint and format check job (cargo fmt, clippy, file size check)
- Change detection job for smarter CI skipping on docs-only changes
- Version modification check to prevent manual version changes in PRs
- Changelog PR release mode for manual releases via pull request workflow
- Crates.io publish step in release pipeline
- RUSTFLAGS=-Dwarnings for strict compilation checks
