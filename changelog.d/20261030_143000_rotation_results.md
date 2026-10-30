---
bump: patch
---

### Changed
- Mark `left_rotate_core` and `right_rotate_core` results as `must_use` so callers
  receive a diagnostic when they discard the new subtree root after a rotation.

### Added
- Compile-fail documentation tests for ignored rotation results.
