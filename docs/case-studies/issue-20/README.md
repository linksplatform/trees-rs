# Case Study: Issue #20 — CI/CD Version Regression and Release Failure

## Overview

**Issue:** [#20 — Something wrong with our CI/CD again](https://github.com/linksplatform/trees-rs/issues/20)
**Affected Run:** [Run 23695940118](https://github.com/linksplatform/trees-rs/actions/runs/23695940118/job/69031223565)
**Date:** 2026-03-28
**Severity:** High — blocked automated releases

---

## Timeline of Events

| Time (UTC) | Event |
|---|---|
| ~2026-03-24 | CI/CD run fails with "Could not parse version" (referenced in issue but not the specific log we analyzed) |
| 2026-03-28T21:20 | Tag `v0.2.0` and GitHub release `v0.2.0` created (unclear how — likely a prior successful bump or manual action) |
| 2026-03-28T22:12 | GitHub release `v0.1.0-beta.1` created (CI created release for old stale version) |
| 2026-03-28T22:40 | Run 23695940118 starts on commit `4e1333a` (push to main) |
| 2026-03-28T22:43 | get-bump-type.rs detects 6 changelog fragments → bump type: `minor` |
| 2026-03-28T22:44 | check-release-needed.rs: `should_release=true`, `skip_bump=false` |
| 2026-03-28T22:44 | version-and-commit.rs runs, computes target version `0.2.0`, detects git tag `v0.2.0` already exists → sets `already_released=true`, `new_version=0.2.0`, exits **without updating Cargo.toml** |
| 2026-03-28T22:44 | get-version.rs reads stale Cargo.toml → outputs `version=0.1.0-beta.1` |
| 2026-03-28T22:44 | publish-crate.rs: "Version 0.1.0-beta.1 already exists on crates.io" — skips (OK) |
| 2026-03-28T22:44 | create-github-release.rs: tries to create release for `v0.1.0-beta.1` → HTTP 422 "Validation Failed" |
| 2026-03-28T22:44 | Error check only looks for `"already exists"` in stderr, doesn't match 422 body → **exits with code 1** |
| 2026-03-28T22:44 | Auto Release job **FAILS** |

---

## Root Causes

### Root Cause 1: Stale Cargo.toml version (version regression)

**What happened:**
The git tag `v0.2.0` was created (by a prior CI run or manual action), but `Cargo.toml` still contained `version = "0.1.0-beta.1"`. The automated pipeline then detected this inconsistency.

**Why it happened:**
`version-and-commit.rs` bumps Cargo.toml and creates the tag atomically. If the process was interrupted or a tag was created externally without updating Cargo.toml (e.g., via manual `git tag`), Cargo.toml and git tags can diverge.

**Evidence:**
- `Cargo.toml` contains `version = "0.1.0-beta.1"` on the main branch
- `git tag` shows both `v0.1.0-beta.1` and `v0.2.0`
- `gh release list` shows both releases

### Root Cause 2: Wrong version passed to create-github-release in auto-release job

**What happened:**
When `version-and-commit.rs` detects `already_released=true` (tag exists), it does NOT update Cargo.toml. However, the subsequent `get-version.rs` step reads the stale Cargo.toml and outputs the old version. The `Create GitHub Release` step in auto-release used `steps.current_version.outputs.version` (from `get-version.rs`) instead of `steps.version.outputs.new_version` (from `version-and-commit.rs`).

**Contrast:** The manual-release job correctly uses `steps.version.outputs.new_version` throughout.

**Evidence (from CI log lines 3782-3797):**
```
version-and-commit.rs: Tag v0.2.0 already exists
version-and-commit.rs: Output: already_released=true
version-and-commit.rs: Output: new_version=0.2.0

get-version.rs: Current version: 0.1.0-beta.1
get-version.rs: Output: version=0.1.0-beta.1

create-github-release.rs: Creating GitHub release for v0.1.0-beta.1...
Error creating release: gh: Validation Failed (HTTP 422)
```

### Root Cause 3: Insufficient error handling in create-github-release.rs

**What happened:**
`create-github-release.rs` only checks for `"already exists"` in stderr to detect a duplicate release. But when GitHub's API returns HTTP 422, the `gh` CLI outputs `"Validation Failed (HTTP 422)"` — not `"already exists"`.

**Fix applied:** Expanded the check to also match `"Validation Failed"` in the combined stderr+stdout output.

---

## Solutions Implemented

### Fix 1: Restore Cargo.toml version to match released tag

Updated `Cargo.toml` version from `0.1.0-beta.1` → `0.2.0` to match the already-released `v0.2.0` tag.

### Fix 2: Use correct version in auto-release Create GitHub Release step

In `.github/workflows/ci.yml`, the auto-release `Create GitHub Release` step now uses `steps.version.outputs.new_version` (computed by `version-and-commit.rs`, which checks git tags) as the authoritative version, falling back to `steps.current_version.outputs.version` only if not available.

This makes auto-release consistent with manual-release, which already used `new_version`.

### Fix 3: Robust error handling in create-github-release.rs

Expanded the "release already exists" check to catch the actual GitHub API error message `"Validation Failed"` (HTTP 422), in addition to the literal `"already exists"` string.

---

## Prevention

To prevent similar issues in the future:

1. **Never create git tags manually** without also updating `Cargo.toml` — let the CI pipeline do it atomically.
2. **Add a consistency check** step that validates that `version` in `Cargo.toml` matches the latest git tag (could be added to lint or a separate check step).
3. **Prefer authoritative sources** — `version-and-commit.rs` is the authoritative source for the release version during a CI run; always use its output, not re-reads of Cargo.toml.

---

## Artifacts

- CI run log: [`ci-logs/run-23695940118.log`](../../../ci-logs/run-23695940118.log)
- Affected files: `Cargo.toml`, `.github/workflows/ci.yml`, `scripts/create-github-release.rs`
