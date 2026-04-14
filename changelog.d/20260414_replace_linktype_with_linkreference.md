---
bump: minor
---

### Changed

- **Breaking:** Removed `LinkType` trait entirely — use `LinkReference` from `platform-num` instead
- All trait bounds now use `T: LinkReference` instead of `T: LinkType`
- Replaced all `funty()` calls with `from_byte()` from `LinkReference`
- Re-exported `LinkReference` from `platform_num` in the public API
- Removed direct `num-traits` dependency (now provided transitively via `platform-num`)
- Upgraded `platform-num` from 0.6.0 to 0.7.0
- Added `u128` support via `LinkReference`
