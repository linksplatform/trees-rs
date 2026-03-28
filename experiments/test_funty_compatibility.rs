// Experiment: Verify that removing the `funty` crate dependency from trees-rs
// does NOT break the public `funty()` method API.
//
// Key finding: The `funty` crate and the `funty()` method are different things.
// - `funty` crate: provides `funty::Unsigned` trait
// - `funty()` method: a convenience method on the `LinkType` trait for creating
//   small integer values from u8
//
// PR #9 removed the `funty` crate dependency but PRESERVED the `funty()` method
// on the `LinkType` trait. The `funty()` method now uses `num_traits::Unsigned`
// instead of `funty::Unsigned` for the trait bound, but the method signature
// and behavior are identical.
//
// See tests/funty_compatibility.rs for the runnable test version.
