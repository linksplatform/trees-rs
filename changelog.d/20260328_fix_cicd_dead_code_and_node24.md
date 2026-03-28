---
bump: patch
---

### Fixed
- Fix Auto Release job failure caused by `dead_code` error in `scripts/version-and-commit.rs` when compiled with `RUSTFLAGS=-Dwarnings`

### Changed
- Upgrade `actions/checkout` from v4 to v6 (Node.js 24 support)
- Upgrade `actions/cache` from v4 to v5 (Node.js 24 support)
- Upgrade `peter-evans/create-pull-request` from v7 to v8 (Node.js 24 support)
