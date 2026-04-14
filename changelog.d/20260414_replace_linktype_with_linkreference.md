---
bump: minor
---

### Changed

- Replaced `LinkType` trait internals with `LinkReference` from `platform-num` 0.7.0
- `LinkType` is now a simple supertrait of `LinkReference` (backward-compatible)
- Replaced all `funty()` calls with `from_byte()` from `LinkReference`
- Re-exported `LinkReference` from `platform_num` in the public API
- Removed direct `num-traits` dependency (now provided transitively via `platform-num`)
- Upgraded `platform-num` from 0.6.0 to 0.7.0
- Added `u128` support via `LinkReference`
