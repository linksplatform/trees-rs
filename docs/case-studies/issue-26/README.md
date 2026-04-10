# Case Study: Issue #26 — Can we remove `num-traits` direct dependency from `platform-trees`?

## Issue Summary

**Title:** If num-traits already imported in platform-num, can we remove direct dependency safely to make it always the same as in platform-num?

**URL:** https://github.com/linksplatform/trees-rs/issues/26

**Status:** Open (no comments)

---

## Timeline / Sequence of Events

1. `platform-trees` was developed with both `num-traits` and `platform-num` as direct dependencies.
2. `platform-num` v0.5.0 internally depends on `num-traits = "0.2.19"`.
3. `platform-trees` declares `num-traits = "0.2"` (any 0.2.x), which currently resolves to `0.2.19`.
4. Issue #26 was raised: is the direct `num-traits` dependency redundant, and can it be removed to ensure version consistency?

---

## All Requirements from the Issue

1. **Investigate**: Can we remove the direct `num-traits` dependency from `platform-trees`?
2. **Safety**: Is the removal safe — won't it cause version mismatches?
3. **Consistency**: Can we ensure `num-traits` version is always the same as what `platform-num` uses?
4. **Deep case study**: Compile all data, reconstruct timeline, find root causes, propose solutions.
5. **Check existing components/libraries** that solve similar problems.
6. **If related to another repo**: File an issue there with reproducible example.

---

## Root Cause Analysis

### What the code actually does

`src/link_type.rs` directly imports two traits from `num-traits`:

```rust
use num_traits::{FromPrimitive, Unsigned};
```

These are used as trait bounds on `LinkType`:

```rust
pub trait LinkType: Number + Unsigned + Sized + TryFrom<u8> + FromPrimitive { ... }
```

### Why direct removal fails

**Rust requires explicit direct dependencies.** Transitive dependencies are not accessible in code unless the intermediate crate re-exports them. Attempting to remove `num-traits` from `Cargo.toml` causes:

```
error[E0432]: unresolved import `num_traits`
 --> src/link_type.rs:1:5
  |
1 | use num_traits::{FromPrimitive, Unsigned};
  |     ^^^^^^^^^^ use of unresolved module or unlinked crate `num_traits`
```

This is a fundamental Rust/Cargo design: each crate must explicitly declare its dependencies.

### The version consistency risk

Current state:
- `platform-trees`: `num-traits = "0.2"` (any 0.2.x compatible)
- `platform-num` v0.5.0: `num-traits = "0.2.19"` (exact 0.2.x)

Currently Cargo resolves both to `0.2.19`, but:
- If `platform-num` upgrades to `num-traits = "0.3"` in a future version, and `platform-trees` still declares `"0.2"`, they'd use **different incompatible versions** of `num-traits`.
- In Rust, `num_traits_0_2::Unsigned` and `num_traits_0_3::Unsigned` are different types — code that combines them won't compile.

### Does `platform-num` re-export `num-traits`?

**No.** `platform-num`'s `src/lib.rs` exports only:
```rust
pub use imp::{LinkReference, MaxValue, Number, SignedNumber, ToSigned};
```

The `num-traits` traits (`Unsigned`, `FromPrimitive`, etc.) are used internally but not re-exported.

---

## Dependency Graph

```
platform-trees v0.2.0
├── num-traits v0.2.19           ← DIRECT dependency (required)
└── platform-num v0.5.0
    └── num-traits v0.2.19 (*)   ← same resolved version (currently)
```

---

## Proposed Solutions

### Solution A (Implemented): Pin `num-traits` to exact version used by `platform-num`

Change `Cargo.toml` from:
```toml
num-traits = "0.2"
```
to:
```toml
num-traits = "=0.2.19"
```

**Pros:** Ensures version always matches `platform-num`. Simple change.
**Cons:** Need to update this pin whenever `platform-num` upgrades `num-traits`. More restrictive for downstream users.

### Solution B: Use `=` constraint matching `platform-num`'s constraint

Change to:
```toml
num-traits = "0.2.19"
```
(semver: >=0.2.19, <0.3.0)

**Pros:** Stays flexible within 0.2.x while ensuring minimum compatible version. Best practice.
**Cons:** Technically could resolve to a different minor bump than `platform-num`, but in practice SemVer guarantees compatibility within 0.2.x.

### Solution C (Recommended for future): File issue on `platform-num` to re-export `num-traits` traits

Ask `platform-num` maintainers to add:
```rust
pub use num_traits::{FromPrimitive, Unsigned, /* other used traits */};
```

This would allow `platform-trees` to remove `num-traits` entirely and use:
```rust
use platform_num::{Number, FromPrimitive, Unsigned};
```

**Pros:** True single-source-of-truth, guaranteed version consistency forever.
**Cons:** Requires `platform-num` to expose those re-exports; changes its API surface; may require a semver bump.

---

## Implementation

**Chosen approach (Solution B):** Update `platform-trees`'s `num-traits` constraint to match `platform-num`'s pinned version. This is the minimal, safe change that directly answers the issue's concern about version consistency.

See: `Cargo.toml` line 22 — changed from `num-traits = "0.2"` to `num-traits = "0.2.19"`.

**Filed issue on `platform-num`:** Requested re-export of `num-traits` traits to enable future removal of the direct dependency. See: https://github.com/linksplatform/Numbers/issues/141

---

## Similar Problems / Known Solutions

- **`pub use` re-exports**: Standard Rust pattern — crates re-export their dependencies' public API to avoid requiring users to specify transitive deps. Example: `tokio` re-exports `bytes::Bytes`.
- **Cargo's `[patch]` section**: Used to override transitive dep versions, but not applicable here.
- **`extern crate` declarations**: Pre-2018 edition pattern, not relevant now.
- **Facade crates**: A crate that bundles and re-exports multiple related crates (e.g., `serde` with its `derive` feature). Could be applied here if `platform-num` becomes a facade for the whole num-traits ecosystem.
