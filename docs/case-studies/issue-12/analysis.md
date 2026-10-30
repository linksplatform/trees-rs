# Case Study: Issue #12 — CI/CD Still Fails

## Summary

CI/CD pipeline run [#23693550016](https://github.com/linksplatform/trees-rs/actions/runs/23693550016) failed with **1 error and 7 warnings**. The error caused the Auto Release job to fail, and the warnings indicate upcoming Node.js 20 deprecation.

## Timeline of Events

| Time (UTC) | Event |
|---|---|
| 2026-03-28 20:20:56 | CI run triggered on push to `main` (merge of PR #11) |
| 2026-03-28 20:20:58 | Detect Changes job starts, completes successfully |
| 2026-03-28 20:21:58 | Lint and Format Check starts, passes |
| 2026-03-28 20:21:59 | Test jobs start on ubuntu, macos, windows — all pass (99 tests each) |
| 2026-03-28 20:22:55 | Build Package starts, completes successfully |
| 2026-03-28 20:23:14 | Auto Release job starts |
| 2026-03-28 20:24:03 | `rust-script` installed, git configured |
| 2026-03-28 20:24:12 | Bump type determined: `minor` (from 3 changelog fragments) |
| 2026-03-28 20:24:41 | `check-release-needed.rs` determines `should_release=true` |
| 2026-03-28 20:24:44 | **FAILURE**: `version-and-commit.rs` compilation fails with `dead_code` error |
| 2026-03-28 20:24:44 | Auto Release job exits with code 1 |

## Root Cause Analysis

### Error 1: Auto Release Failure (CRITICAL)

**Direct cause**: The `scripts/version-and-commit.rs` script failed to compile.

**Root cause**: The `Version` struct at line 112-117 has a `pre_release` field that is parsed during version extraction but never read by any method. The `bump()` method (the only consumer of the struct) computes the new version from `major`, `minor`, and `patch` fields only.

```rust
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Option<String>,  // <-- parsed but never read
}
```

The CI workflow sets `RUSTFLAGS: -Dwarnings` globally (line 38 of `ci.yml`), which promotes all warnings to errors. The `dead_code` lint detects the unused field and emits a warning, which `-Dwarnings` escalates to a compilation error.

**Exact error from CI logs (line 3558-3572)**:
```
error[E0050]: field `pre_release` is never read
  --> scripts/version-and-commit.rs:116:5
     = note: `-D dead-code` implied by `-D warnings`
```

**Why this wasn't caught earlier**: The `pre_release` field was added in commit `f41eddc` ("Fix version parsing to support pre-release semver versions") as part of issue #10 fix. The field was added to `version-and-commit.rs` for consistency with `bump-version.rs`, but unlike `bump-version.rs`, the `version-and-commit.rs` script has no `to_string()` method that uses the field. The `rust-script` compilation happens at runtime in CI (not during local development where `RUSTFLAGS` may not include `-Dwarnings`), so the issue only manifested when CI attempted an auto-release on push to `main`.

**Contributing factor**: The `scripts/` directory contains Rust scripts executed via `rust-script`, which compiles them on-the-fly. These scripts are not part of the main `cargo build` and therefore are not checked by the Lint and Format Check job. There is no CI step that pre-compiles or lints the scripts directory.

### Warning: Node.js 20 Deprecation (7 warnings)

All 7 warnings are identical in nature — GitHub Actions running on Node.js 20 are deprecated:

| Job | Actions Using Node.js 20 |
|---|---|
| Detect Changes | `actions/checkout@v4` |
| Test (ubuntu-latest) | `actions/cache@v4`, `actions/checkout@v4` |
| Test (macos-latest) | `actions/cache@v4`, `actions/checkout@v4` |
| Test (windows-latest) | `actions/cache@v4`, `actions/checkout@v4` |
| Lint and Format Check | `actions/cache@v4`, `actions/checkout@v4` |
| Build Package | `actions/cache@v4`, `actions/checkout@v4` |
| Auto Release | `actions/checkout@v4` |

**Timeline**: Node.js 20 will be forced to Node.js 24 on June 2, 2026, and removed from runners on September 16, 2026.

## Solution Applied

### Fix 1: Suppress dead_code warning on `pre_release` field

Added `#[allow(dead_code)]` annotation to the `pre_release` field in `scripts/version-and-commit.rs`. The field is needed for correct regex parsing (the pre-release portion of a semver string must be captured to avoid mismatched quotes), but its value is intentionally unused since `bump()` always produces a clean `major.minor.patch` version.

**Alternative considered**: Removing the field entirely. Rejected because:
1. The regex pattern still captures group 4 (the pre-release suffix) — removing the field would require changing the parsing logic
2. Future enhancements may need the pre-release info (e.g., for smarter bump decisions)
3. The `bump-version.rs` script has the same struct with a `to_string()` that uses the field — consistency is valuable

### Fix 2: Upgrade GitHub Actions to Node.js 24

| Action | Old Version | New Version | Node.js |
|---|---|---|---|
| `actions/checkout` | v4 | v6 | 24 |
| `actions/cache` | v4 | v5 | 24 |
| `peter-evans/create-pull-request` | v7 | v8 | 24 |

## Preventive Recommendations

1. **Add script linting to CI**: Add a step that compiles all `scripts/*.rs` files with `RUSTFLAGS=-Dwarnings` as a pre-check, so dead_code issues in scripts are caught before they reach release jobs.

2. **Pin action versions to SHA**: Instead of major version tags (e.g., `@v6`), pin to specific commit SHAs for reproducibility and security.

3. **Dependabot for Actions**: Enable Dependabot to automatically propose updates when GitHub Actions release new versions.

## Data Files

- `ci-run-23693550016.log` — Full CI run log (3588 lines)

## References

- GitHub Actions Node.js 20 deprecation: https://github.blog/changelog/2025-09-19-deprecation-of-node-20-on-github-actions-runners/
- actions/checkout v6 release: https://github.com/actions/checkout/releases
- actions/cache v5: https://github.com/actions/cache/releases
- peter-evans/create-pull-request v8: https://github.com/peter-evans/create-pull-request/releases
