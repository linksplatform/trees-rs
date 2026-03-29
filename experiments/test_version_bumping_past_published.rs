#!/usr/bin/env rust-script
//! Test that version bumping logic correctly skips past already-published versions.
//!
//! This experiment verifies the core logic from issue #22:
//! Given a current version and a set of "published" versions, the bump must produce
//! a version strictly greater than the max published version.
//!
//! ```cargo
//! [dependencies]
//! ```

fn bump(major: u32, minor: u32, patch: u32, bump_type: &str) -> (u32, u32, u32) {
    match bump_type {
        "major" => (major + 1, 0, 0),
        "minor" => (major, minor + 1, 0),
        _ => (major, minor, patch + 1),
    }
}

fn ensure_version_exceeds(
    version: (u32, u32, u32),
    max_published: Option<(u32, u32, u32)>,
) -> (u32, u32, u32) {
    let (major, minor, patch) = version;

    if let Some((pub_major, pub_minor, pub_patch)) = max_published {
        if (major, minor, patch) <= (pub_major, pub_minor, pub_patch) {
            // Set to max_published + patch 1 to exceed it
            println!(
                "  {}.{}.{} <= {}.{}.{}, adjusting to {}.{}.{}",
                major, minor, patch,
                pub_major, pub_minor, pub_patch,
                pub_major, pub_minor, pub_patch + 1
            );
            return (pub_major, pub_minor, pub_patch + 1);
        }
    }

    (major, minor, patch)
}

fn test_case(
    name: &str,
    current: (u32, u32, u32),
    bump_type: &str,
    max_published: Option<(u32, u32, u32)>,
    expected: (u32, u32, u32),
) {
    let bumped = bump(current.0, current.1, current.2, bump_type);
    let final_ver = ensure_version_exceeds(bumped, max_published);

    let passed = final_ver == expected;
    let status = if passed { "PASS" } else { "FAIL" };

    println!(
        "[{}] {}: {}.{}.{} --{}-> {}.{}.{} (max_pub={}) => {}.{}.{} (expected {}.{}.{})",
        status,
        name,
        current.0, current.1, current.2,
        bump_type,
        bumped.0, bumped.1, bumped.2,
        max_published.map_or("none".to_string(), |(a,b,c)| format!("{}.{}.{}", a, b, c)),
        final_ver.0, final_ver.1, final_ver.2,
        expected.0, expected.1, expected.2,
    );

    if !passed {
        std::process::exit(1);
    }
}

fn main() {
    println!("=== Testing version bumping past published versions ===\n");

    // Case 1: Normal bump, version exceeds published -> no adjustment needed
    test_case(
        "normal minor bump exceeds published",
        (0, 2, 0), "minor", Some((0, 2, 0)),
        (0, 3, 0),
    );

    // Case 2: Patch bump produces 0.2.1 which exceeds 0.2.0 -> OK
    test_case(
        "patch bump past published",
        (0, 2, 0), "patch", Some((0, 2, 0)),
        (0, 2, 1),
    );

    // Case 3: Minor bump from 0.1.0 gives 0.2.0 which equals published 0.2.0 -> must exceed
    test_case(
        "minor bump equal to published",
        (0, 1, 0), "minor", Some((0, 2, 0)),
        (0, 2, 1),
    );

    // Case 4: Major bump, no conflict
    test_case(
        "major bump no conflict",
        (0, 5, 3), "major", Some((0, 5, 3)),
        (1, 0, 0),
    );

    // Case 5: No published versions at all
    test_case(
        "first release",
        (0, 1, 0), "patch", None,
        (0, 1, 1),
    );

    // Case 6: Published version much higher - should jump to published+1
    test_case(
        "published ahead - jumps to max_published+1",
        (0, 1, 0), "patch", Some((0, 5, 0)),
        (0, 5, 1),
    );

    // Case 7: THE ACTUAL BUG SCENARIO
    // Cargo.toml has 0.2.0, published is 0.2.0, minor bump gives 0.3.0
    // 0.3.0 > 0.2.0 so no adjustment needed
    test_case(
        "issue-22 scenario: Cargo.toml at published version",
        (0, 2, 0), "minor", Some((0, 2, 0)),
        (0, 3, 0),
    );

    // Case 8: Edge case - what if version-and-commit somehow produces the same version
    // (this was the actual bug: current=0.2.0, bump produced 0.2.0 because of stale state)
    test_case(
        "issue-22 actual bug: bump produces same as published",
        (0, 1, 5), "patch", Some((0, 2, 0)),
        (0, 2, 1),
    );

    println!("\n=== All tests passed! ===");
}
