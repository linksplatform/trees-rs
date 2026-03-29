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
| 2026-03-28T21:16 | Tag `v0.2.0` and GitHub release `v0.2.0` created by CI; `Cargo.toml` **not updated** (root cause 1) |
| 2026-03-28T22:08 | GitHub release `v0.1.0-beta.1` created by CI (stale Cargo.toml version propagated) |
| 2026-03-28T22:40 | Run 23695940118 starts on commit `4e1333a` (push to main) |
| 2026-03-28T22:43 | `get-bump-type.rs` detects changelog fragments → bump type: `minor` |
| 2026-03-28T22:44 | `check-release-needed.rs`: `should_release=true`, `skip_bump=false` |
| 2026-03-28T22:44 | `version-and-commit.rs` runs, computes target version `0.2.0`, detects git tag `v0.2.0` already exists → sets `already_released=true`, `new_version=0.2.0`, exits **without updating `Cargo.toml`** |
| 2026-03-28T22:44 | `get-version.rs` reads stale `Cargo.toml` → outputs `version=0.1.0-beta.1` |
| 2026-03-28T22:44 | `publish-crate.rs`: "Version `0.1.0-beta.1` already exists on crates.io" — skips (OK) |
| 2026-03-28T22:44 | `create-github-release.rs`: tries to create release for `v0.1.0-beta.1` → HTTP 422 "Validation Failed" |
| 2026-03-28T22:44 | Error check only matches `"already exists"` in stderr, doesn't match 422 body → **exits with code 1** |
| 2026-03-28T22:44 | Auto Release job **FAILS** |

---

## Root Causes

### Root Cause 1 (Primary): `version-and-commit.rs` does not update `Cargo.toml` when tag already exists

**What happened:**
`version-and-commit.rs` bumps the version, updates `Cargo.toml`, and creates a git tag atomically. However, when it detects that the computed tag (`v0.2.0`) already exists, it returns early — **skipping the `Cargo.toml` update**. This means `Cargo.toml` permanently stays at the old pre-release version (`0.1.0-beta.1`) even though `v0.2.0` is already published.

**Why it matters:**
Every subsequent CI run reads the stale `Cargo.toml` version. The downstream steps (`get-version.rs`, `publish-crate.rs`, `create-github-release.rs`) all propagate the wrong version, leading to cascading failures.

**Evidence:**
- `Cargo.toml` on `main` contained `version = "0.1.0-beta.1"`
- `git tag` showed both `v0.1.0-beta.1` and `v0.2.0`
- Both versions published on crates.io confirmed by API

**Fix:** When `already_released=true` and `Cargo.toml` has a stale version, `version-and-commit.rs` now syncs `Cargo.toml` to match the existing tag and commits + pushes the change.

---

### Root Cause 2: Wrong version passed to `Create GitHub Release` in auto-release job

**What happened:**
The `Create GitHub Release` step in the auto-release job used `steps.current_version.outputs.version` (from `get-version.rs`, which reads the stale `Cargo.toml`) instead of `steps.version.outputs.new_version` (from `version-and-commit.rs`, which is tag-checked and authoritative).

**Contrast:** The manual-release job correctly uses `steps.version.outputs.new_version` throughout.

**Evidence (from CI log):**
```
version-and-commit.rs: Tag v0.2.0 already exists
version-and-commit.rs: Output: already_released=true
version-and-commit.rs: Output: new_version=0.2.0

get-version.rs: Current version: 0.1.0-beta.1
get-version.rs: Output: version=0.1.0-beta.1

create-github-release.rs: Creating GitHub release for v0.1.0-beta.1...
Error creating release: gh: Validation Failed (HTTP 422)
```

**Fix:** Auto-release `Create GitHub Release` step now uses `steps.version.outputs.new_version` as the primary source, with fallback to `steps.current_version.outputs.version` — consistent with manual-release.

---

### Root Cause 3: Insufficient error handling in `create-github-release.rs`

**What happened:**
`create-github-release.rs` only checked for `"already exists"` in stderr to detect a duplicate release. When GitHub's API returns HTTP 422, the `gh` CLI outputs `"Validation Failed (HTTP 422)"` — not `"already exists"`.

**Fix:** Expanded the check to also match `"Validation Failed"` and `"already_exists"` in the combined stderr+stdout output.

---

## Solutions Implemented

### Fix 1 (Primary): Sync `Cargo.toml` to existing tag in `version-and-commit.rs`

When `version-and-commit.rs` detects that the computed tag already exists, it now checks if `Cargo.toml` reflects that version. If not (stale pre-release version), it updates `Cargo.toml` and commits + pushes the sync, ensuring the working tree always reflects the latest release.

This is the root fix — it prevents the version regression from persisting across CI runs.

### Fix 2: Use correct version in auto-release `Create GitHub Release` step

In `.github/workflows/ci.yml`, the auto-release `Create GitHub Release` step now uses `steps.version.outputs.new_version` (tag-checked, authoritative) as the primary version, falling back to `steps.current_version.outputs.version` only if not set.

### Fix 3: Robust error handling in `create-github-release.rs`

Expanded the "release already exists" check to also catch `"Validation Failed"` (HTTP 422 from GitHub API), so duplicate release attempts skip gracefully instead of failing the job.

---

## Prevention

To prevent similar issues in the future:

1. **Let the CI pipeline manage versions atomically** — `version-and-commit.rs` now handles the sync case; never create git tags manually without also updating `Cargo.toml`.
2. **Prefer authoritative sources** — `version-and-commit.rs` is the authoritative source for release version during a CI run; always use its output, not re-reads of `Cargo.toml`.
3. **Defensive error matching** — When checking external API error responses, match on the actual returned error format, not assumed strings.

---

## Artifacts

- CI run log: [`ci-logs/run-23695940118.log`](../../../ci-logs/run-23695940118.log)
- Affected files: `scripts/version-and-commit.rs`, `.github/workflows/ci.yml`, `scripts/create-github-release.rs`
