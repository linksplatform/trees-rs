// Experiment: Verify that replacing `LinkType::funty()` with
// `LinkReference::from_byte()` works correctly.
//
// History:
// - `funty` crate was removed in PR #9, but `funty()` method was preserved
// - Issue #30 replaced the `funty()` method with `from_byte()` from
//   `LinkReference` (platform-num), unifying the trait hierarchy
//
// See tests/funty_compatibility.rs for the runnable test version.
