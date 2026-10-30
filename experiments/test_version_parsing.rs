#!/usr/bin/env rust-script
//! Test version parsing with pre-release semver versions
//! Verifies the fix for issue #10 - CI/CD version parsing failure
//!
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

use regex::Regex;

struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Option<String>,
}

impl Version {
    fn parse(content: &str) -> Option<Version> {
        let re = Regex::new(r#"(?m)^version\s*=\s*"(\d+)\.(\d+)\.(\d+)(?:-([^"]+))?""#).ok()?;
        let caps = re.captures(content)?;
        Some(Version {
            major: caps.get(1)?.as_str().parse().ok()?,
            minor: caps.get(2)?.as_str().parse().ok()?,
            patch: caps.get(3)?.as_str().parse().ok()?,
            pre_release: caps.get(4).map(|m| m.as_str().to_string()),
        })
    }

    fn bump(&self, bump_type: &str) -> String {
        match bump_type {
            "major" => format!("{}.0.0", self.major + 1),
            "minor" => format!("{}.{}.0", self.major, self.minor + 1),
            _ => format!("{}.{}.{}", self.major, self.minor, self.patch + 1),
        }
    }

    fn to_string(&self) -> String {
        match &self.pre_release {
            Some(pre) => format!("{}.{}.{}-{}", self.major, self.minor, self.patch, pre),
            None => format!("{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}

fn test_case(content: &str, expected_version: &str, expected_minor_bump: &str) {
    match Version::parse(content) {
        Some(v) => {
            let actual = v.to_string();
            let bumped = v.bump("minor");
            let version_ok = actual == expected_version;
            let bump_ok = bumped == expected_minor_bump;
            println!(
                "{} parse '{}' => '{}' (expected '{}')",
                if version_ok { "PASS" } else { "FAIL" },
                content.trim(),
                actual,
                expected_version
            );
            println!(
                "{} minor bump '{}' => '{}' (expected '{}')",
                if bump_ok { "PASS" } else { "FAIL" },
                actual,
                bumped,
                expected_minor_bump
            );
        }
        None => {
            println!("FAIL parse '{}' => None (expected '{}')", content.trim(), expected_version);
        }
    }
    println!();
}

fn main() {
    println!("=== Version Parsing Tests (Issue #10 Fix) ===\n");

    // Standard version - should still work
    test_case(
        "version = \"1.2.3\"\n",
        "1.2.3",
        "1.3.0",
    );

    // THE BUG: Pre-release version that caused CI failure
    test_case(
        "version = \"0.1.0-beta.1\"\n",
        "0.1.0-beta.1",
        "0.2.0",
    );

    // Other pre-release formats
    test_case(
        "version = \"1.0.0-alpha\"\n",
        "1.0.0-alpha",
        "1.1.0",
    );

    test_case(
        "version = \"2.0.0-rc.1\"\n",
        "2.0.0-rc.1",
        "2.1.0",
    );

    // Test with surrounding content (realistic Cargo.toml)
    let cargo_toml = r#"[package]
name = "platform-trees"
version = "0.1.0-beta.1"
edition = "2021"
"#;
    test_case(cargo_toml, "0.1.0-beta.1", "0.2.0");

    println!("=== All tests completed ===");
}
