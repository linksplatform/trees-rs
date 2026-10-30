# Case Study: Issue #10 - CI/CD Version Parsing Failure with Pre-Release Semver Versions

## Summary

The CI/CD auto-release pipeline fails when `Cargo.toml` contains a pre-release semver version
(e.g., `0.1.0-beta.1`). The version parsing regex in release scripts only matches standard
`X.Y.Z` format and rejects versions with pre-release suffixes.

## CI/CD Run Reference

- **Failed run**: https://github.com/linksplatform/trees-rs/actions/runs/23493592626
- **Date**: 2026-03-24T14:04–14:08 UTC
- **Trigger**: Push to `main` branch
- **Failed job**: `Auto Release`
- **Failed step**: `Collect changelog and bump version` (version-and-commit.rs)
- **Full logs**: [ci-run-23493592626.txt](./ci-run-23493592626.txt)

## Timeline of Events

1. Push to `main` triggers the CI/CD pipeline
2. `detect-changes`, `changelog`, `lint`, `test`, and `build` jobs all pass successfully
3. `auto-release` job begins:
   - `get-bump-type.rs` detects 2 changelog fragments, determines `minor` bump
   - `check-release-needed.rs` confirms release is needed (`should_release=true`)
   - `version-and-commit.rs` attempts to parse version from `Cargo.toml`
4. **FAILURE**: `version-and-commit.rs` exits with error:
   ```
   Error: Could not parse version from ./Cargo.toml
   ```

## Root Cause

The `Version::parse()` method in `scripts/version-and-commit.rs` (line 120) uses a regex that
only matches strict `X.Y.Z` format:

```rust
// OLD (broken) regex:
let re = Regex::new(r#"(?m)^version\s*=\s*"(\d+)\.(\d+)\.(\d+)""#).ok()?;
```

The actual version in `Cargo.toml` is:
```toml
version = "0.1.0-beta.1"
```

The regex expects a closing `"` immediately after the third number group (`\d+`), but the
pre-release suffix `-beta.1` appears between the patch number and the closing quote. The regex
fails to match, `Version::parse()` returns `None`, and the script exits with an error.

### Affected Scripts

| Script | Regex | Status |
|--------|-------|--------|
| `version-and-commit.rs` | `(\d+)\.(\d+)\.(\d+)"` | **BROKEN** - Failed in CI |
| `bump-version.rs` | `(\d+)\.(\d+)\.(\d+)"` | **BROKEN** - Same pattern |
| `check-release-needed.rs` | `([^"]+)"` | OK - Captures full version string |
| `get-version.rs` | `([^"]+)"` | OK - Captures full version string |
| `publish-crate.rs` | `([^"]+)"` | OK - Captures full version string |

The inconsistency between scripts is the core issue: some scripts use a strict `X.Y.Z` pattern
while others use a permissive `[^"]+` pattern. When `Cargo.toml` gained a pre-release version,
the strict scripts broke.

## Fix

Updated the regex in both affected scripts to support optional pre-release suffixes:

```rust
// NEW (fixed) regex:
let re = Regex::new(r#"(?m)^version\s*=\s*"(\d+)\.(\d+)\.(\d+)(?:-([^"]+))?""#).ok()?;
```

The `(?:-([^"]+))?` group optionally captures the pre-release suffix (everything after `-`
up to the closing `"`). The `Version` struct was also extended to store the `pre_release` field.

When bumping, pre-release suffixes are stripped (following semver convention: bumping a
pre-release produces the next release version).

## Lessons Learned

1. **Semver compliance**: Version parsing in CI/CD scripts must handle the full semver spec,
   including pre-release (`-alpha.1`) and build metadata (`+build.123`) suffixes.
2. **Consistency**: All scripts parsing the same data should use the same regex/logic.
   The divergence between strict and permissive patterns created a latent bug.
3. **Testing**: Version parsing should be tested with realistic version strings including
   edge cases like pre-release versions before deploying to CI.
