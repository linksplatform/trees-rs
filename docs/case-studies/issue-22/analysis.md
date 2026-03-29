# Issue #22: Release pipeline must always publish a version greater than what exists on crates.io

## Problem Statement

When the auto-release pipeline runs and encounters a version that already exists on crates.io,
it treats this as a success ("this is OK") and exits cleanly. This is a false positive: the
pipeline should never silently accept that a version was not published. If the version already
exists, the pipeline must bump to a higher version and publish that instead.

## Timeline of Events (CI Run 23707269823)

1. **2026-03-29T10:45:00Z** - Auto Release job starts after successful build
2. **2026-03-29T10:46:05Z** - `check-release-needed.rs` outputs `should_release=true`, `skip_bump=false`
   - 7 changelog fragments exist, so `has_fragments=true`
3. **2026-03-29T10:46:08Z** - `version-and-commit.rs` determines bump type is `minor` (0.2.0 -> 0.3.0... but wait)
   - Actually, current version in Cargo.toml is already `0.2.0`
   - The bump from `0.2.0` with `minor` would produce `0.3.0`
   - BUT the script finds `Tag v0.3.0` does NOT exist... however `Tag v0.2.0` DOES exist
   - The log shows: **"Tag v0.2.0 already exists"** - this means the script computed `0.2.0` as new_version
   - This happens because Cargo.toml already has `0.2.0`, and the previous release cycle already
     created the tag but the fragments were not cleaned up
4. **2026-03-29T10:46:10Z** - `publish-crate.rs` attempts to publish `platform-trees@0.2.0`
5. **2026-03-29T10:46:11Z** - crates.io responds that version already exists
6. **Outcome**: `publish_result=already_exists` - treated as success, but nothing was actually released

## Root Cause Analysis

### Root Cause 1: `publish-crate.rs` treats "already exists" as success

In `publish-crate.rs` lines 170-172:
```rust
if combined.contains("already uploaded") || combined.contains("already exists") {
    println!("Version {} already exists on crates.io - this is OK", version);
    set_output("publish_result", "already_exists");
}
```

The script exits with code 0 (success) when a version already exists. This was originally
intended as a "soft failure" to handle race conditions, but it masks the real problem: the
pipeline is trying to publish a version that's already published.

**Fix**: Exit with failure (code 1) when version already exists. The pipeline should never
attempt to publish an already-published version - that indicates a bug in the version
bumping logic.

### Root Cause 2: `version-and-commit.rs` doesn't bump past existing tags

When `version-and-commit.rs` computes the new version and finds the tag already exists (line 281),
it enters a "sync" path that only updates Cargo.toml to match the existing tag but does NOT
try a higher version. The script should keep bumping until it finds an unpublished version.

**Fix**: When the computed new version already has a tag, keep incrementing (patch bump) until
finding a version without an existing tag and not published on crates.io.

### Root Cause 3: No crates.io version check before publishing

The pipeline checks crates.io in `check-release-needed.rs` but only for the current Cargo.toml
version (to decide if release is needed). It does NOT check whether the *new* bumped version
is already published. The authoritative check should query the maximum published version and
ensure the new version is strictly greater.

**Fix**: Add a function to `check-release-needed.rs` that queries the latest published version
from crates.io and outputs it, so `version-and-commit.rs` can ensure the bump target exceeds it.

### Root Cause 4: Stale changelog fragments

The 7 changelog fragments from previous releases were not cleaned up after the v0.2.0 release.
This caused `check-release-needed.rs` to think a new release was needed (`has_fragments=true`)
even though the fragments were already processed.

**Note**: This is actually the trigger for the whole issue - if fragments had been cleaned,
`has_fragments` would be `false` and the pipeline would have checked the current version
against crates.io instead.

## Sequence Diagram

```
check-release-needed.rs
  |-- has_fragments=true (stale fragments from v0.2.0)
  |-- should_release=true, skip_bump=false
  v
version-and-commit.rs
  |-- current version: 0.2.0 (already at v0.2.0)
  |-- bump minor: 0.2.0 -> 0.3.0? NO - bump reads current as 0.2.0
  |-- Wait: the bump computes from Cargo.toml version 0.2.0
  |-- With fragments requesting "minor": 0.2.0 minor bump = 0.3.0
  |-- But CI log says "Tag v0.2.0 already exists" which means new_version=0.2.0
  |-- This means the script somehow computed 0.2.0 as the new version
  |-- LIKELY: skip_bump was not properly propagated, or Cargo.toml was pre-bumped
  v
publish-crate.rs
  |-- Tries to publish 0.2.0
  |-- crates.io: "already uploaded"
  |-- Script: "this is OK" (EXIT 0) <-- BUG: should be failure
  v
create-github-release.rs
  |-- Release v0.2.0 already exists, skipping
```

## Solutions Implemented

### 1. `publish-crate.rs`: Fail on "already exists"
- Changed `already_exists` from exit 0 to exit 1
- Added `--allow-existing` flag for backwards compatibility if needed
- Clear error message explaining the version must be bumped

### 2. `version-and-commit.rs`: Bump past existing versions
- Added crates.io version checking (query max published version)
- When computed version already has a tag OR is already on crates.io, keep bumping patch
- Ensures the released version is always strictly greater than the latest published version

### 3. `check-release-needed.rs`: Output max published version
- Added query for the latest version published on crates.io
- Outputs `max_published_version` for downstream scripts to use
- Provides better diagnostic information in logs

### 4. Added verbose/debug logging
- All release scripts now support `--verbose` flag
- Detailed logging of version comparison decisions
- Logs crates.io API responses for debugging

## Related Issues and Prior Art

- Issue #10: CI/CD version parsing failure (pre-release semver) - fixed regex
- Issue #12: Dead code + Node.js 20 deprecation - fixed compilation
- Issue #20: Version regression and release failure (HTTP 422) - fixed tag/Cargo.toml sync

## References

- [Semantic Versioning 2.0.0](https://semver.org/)
- [crates.io API documentation](https://crates.io/docs/api)
- [Cargo publish documentation](https://doc.rust-lang.org/cargo/reference/publishing.html)
- CI Run: https://github.com/linksplatform/trees-rs/actions/runs/23707269823/job/69061263454
