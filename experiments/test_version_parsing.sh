#!/bin/bash
# Test that version parsing regex works with various version formats
# This is a quick experiment to verify the fix for issue #10

set -e

echo "=== Testing version parsing regex ==="
echo ""

# Test cases: version string -> expected match behavior
# Using grep with PCRE to simulate the Rust regex

REGEX='version\s*=\s*"([0-9]+)\.([0-9]+)\.([0-9]+)(?:-([^"]+))?"'

test_version() {
    local input="$1"
    local expected_match="$2"
    local description="$3"

    if echo "$input" | grep -P "$REGEX" > /dev/null 2>&1; then
        if [ "$expected_match" = "yes" ]; then
            echo "PASS: $description"
        else
            echo "FAIL: $description (should NOT match but did)"
        fi
    else
        if [ "$expected_match" = "no" ]; then
            echo "PASS: $description"
        else
            echo "FAIL: $description (should match but didn't)"
        fi
    fi
}

# Standard versions
test_version 'version = "1.2.3"' "yes" "Standard semver 1.2.3"
test_version 'version = "0.1.0"' "yes" "Standard semver 0.1.0"

# Pre-release versions (the bug case)
test_version 'version = "0.1.0-beta.1"' "yes" "Pre-release 0.1.0-beta.1"
test_version 'version = "1.0.0-alpha"' "yes" "Pre-release 1.0.0-alpha"
test_version 'version = "2.0.0-rc.1"' "yes" "Pre-release 2.0.0-rc.1"

# Build metadata
test_version 'version = "1.0.0+build.123"' "yes" "Build metadata 1.0.0+build.123"

echo ""
echo "=== Old regex (broken for pre-release) ==="
OLD_REGEX='version\s*=\s*"([0-9]+)\.([0-9]+)\.([0-9]+)"'

test_version_old() {
    local input="$1"
    local expected_match="$2"
    local description="$3"

    # Use exact match (the old regex would match "0.1.0" part of "0.1.0-beta.1"
    # but the regex needs to match the full quoted string to work correctly)
    if echo "$input" | grep -oP "$OLD_REGEX" | head -1 | grep -q "$input"; then
        echo "OLD REGEX MATCH: $description"
    else
        echo "OLD REGEX NO-MATCH: $description"
    fi
}

# The key test: old regex couldn't parse "0.1.0-beta.1" correctly because
# it expected the closing quote right after the third number
echo 'version = "0.1.0-beta.1"' | grep -oP 'version\s*=\s*"(\d+)\.(\d+)\.(\d+)"' && echo "OLD: Matched (incorrectly strips pre-release)" || echo "OLD: No match"
echo 'version = "0.1.0-beta.1"' | grep -oP 'version\s*=\s*"(\d+)\.(\d+)\.(\d+)(?:-([^"]+))?"' && echo "NEW: Matched correctly" || echo "NEW: No match"

echo ""
echo "All tests completed."
