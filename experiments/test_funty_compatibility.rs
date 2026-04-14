// Experiment: Verify that `LinkReference::from_byte()` works correctly.
//
// History:
// - `funty` crate was removed in PR #9, but `funty()` method was preserved
// - Issue #30 replaced both `LinkType` and `funty()` with `LinkReference`
//   and `from_byte()` from `platform-num`, unifying the trait hierarchy
//
// See tests/funty_compatibility.rs for the runnable test version.
