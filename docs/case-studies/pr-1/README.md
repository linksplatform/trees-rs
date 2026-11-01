# Review of the original v2 proposal

## Scope and requirements

[PR #1](https://github.com/linksplatform/trees-rs/pull/1) originally contained
commit `b259ce8435b00b4925acaa2f6f8f795d2a135796` from May 2023. Its original
description was empty, and the initial timeline, closing issue references,
conversation, inline review comments and reviews contained no linked issue or
required sub-issue.

The actionable request is the
[maintainer's October 30, 2026 comment](https://github.com/linksplatform/trees-rs/pull/1#issuecomment-6523805975):
resolve the conflicts, evaluate the old proposal against current code, and retain
only improvements. The review uses main commit
`ed83a999d29c7f6bdd2a9e1eca1d4e643aef1fc3` as its baseline. Recent merged PRs
[#31](https://github.com/linksplatform/trees-rs/pull/31),
[#25](https://github.com/linksplatform/trees-rs/pull/25), and
[#9](https://github.com/linksplatform/trees-rs/pull/9) explain the current numeric
API, integration test layout, and stable toolchain/release conventions.

## Decisions

| Original proposal | Decision and reason |
| --- | --- |
| Root crate layout and removal of local sibling dependencies | Already achieved on main: the crate is at the repository root and uses published `platform-num`. Preserve this layout. |
| `Unlicense` metadata | Already present on main. Preserve the current description, repository/homepage URLs, edition, and other package metadata. |
| Git `platform-data`, `funty`, and a local `LinkType` bridge with `zero`/`one` helpers | Superseded by main's `platform-num` 0.8 and `LinkReference::from_byte`. Reintroducing them would reverse the completed migration. |
| Nightly feature gates and nightly rustfmt settings | Discard. Main builds on stable Rust, and the new formatting settings require unstable options. Preserve main's formatting. |
| `tap` and the slice-based `new_v2` API | Discard the unfinished prototype. It loses parent/root updates when descending, clears the wrong child in the right-left insertion special case, and discards the replacement subtree root during deletion. Its test does not exercise any of this. |
| Existing list/tree edits | Numeric helper substitutions and formatting are superseded by the current API. Preserve main's documented traits, names, algorithms, and integration tests. |
| Criterion benchmark | Discard: its function registers no benchmark and measures no operation. Avoid adding a dependency without useful measurements. |
| Proptest scaffolding | Discard: the test body contains only comments and makes no assertions. Existing integration tests exercise actual operations without those extra dependencies. |
| Unused `tri!` macro and blanket `deny(unused_must_use)` | Discard the unused macro. Main's CI already denies warnings; a crate-local lint alone cannot warn downstream users about discarded rotation roots. |
| `#[must_use]` on rotations that return a new root | Adopt on the current `left_rotate_core` and `right_rotate_core` methods. Their results must update the caller's subtree root. This provides a targeted downstream diagnostic without replacing the working API. |

Main is merged into `v2` with both histories preserved. All conflict resolutions
retain main's implementations. Only the rotation-result annotations, their
documentation/regression tests, this review, and a patch changelog fragment differ
from main. The fragment triggers the existing automatic release workflow; no
manual package version change is needed.

## Reproduction and verification

The missing diagnostic can be reproduced with either rotation in a generic
function, without allocating storage or executing unsafe operations:

```rust
#![deny(unused_must_use)]
use platform_trees::RecursiveSizeBalancedTree;

unsafe fn rotate(tree: &mut impl RecursiveSizeBalancedTree<usize>, root: usize) {
    unsafe { tree.left_rotate_core(root); }
}
```

Before adding the annotations, the two new `compile_fail` documentation tests
both failed with `Test compiled successfully, but it's marked compile_fail`.
After adding the annotations, discarding either result is rejected under
`deny(unused_must_use)`. Existing rotation tests check correct root usage and
tree links/sizes, including the in-place wrappers.

Local checks use one Cargo build job and disabled development/test debug
information to stay within the container memory limit. Commands:

```sh
export CARGO_BUILD_JOBS=1 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
export RUSTFLAGS=-Dwarnings
cargo fmt --all -- --check
cargo clippy --all-targets --all-features
cargo test --all-features --verbose
cargo test --doc --verbose
cargo build --release --verbose
cargo package --list
```

The baseline passed all 116 integration tests and two documentation tests.
The updated suite adds two compile-fail tests. CI also runs formatting, Clippy,
file-size, changelog and version checks, tests on Linux/macOS/Windows, and a
release build/package check.

| Requirement | Implementation and verification evidence |
| --- | --- |
| Resolve conflicts | Merge main into `v2`, resolve every conflicted path using current implementations, and verify main is an ancestor of the resulting branch. |
| Keep only improvements from the old PR | The decision table accounts for each group of changes; only the targeted rotation diagnostics affect current library code. |
| Preserve existing functionality | No algorithm, public trait signature, dependency, or package metadata changes relative to main; all existing tests remain in place. |
| Verify the retained practice | Both regression tests fail before the annotations and pass afterward; existing runtime rotation tests verify proper use. |
| Prepare review and release | Updated PR title/body, this audit, and the patch changelog fragment. There is no original issue requiring a closing keyword. |
