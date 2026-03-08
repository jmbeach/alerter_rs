---
id: ar-2n0x
status: closed
deps: []
links: []
created: 2026-03-08T00:30:00Z
type: task
priority: 2
parent: ar-erxo
tags: [tdd]
---
# Test: Example binary shows usage for unknown arguments

Test that the showcase binary prints usage information and exits with non-zero status for unknown arguments. File: tests/showcase_usage_test.rs

## Acceptance Criteria

Write tests that FAIL verifying:
1. Running the showcase binary with argument `nonexistent` exits with a non-zero status code.
2. Running the showcase binary with argument `nonexistent` prints a usage message listing available notification types to stderr or stdout.
3. Running the showcase binary with no arguments prints a usage message.

Tests must fail since `examples/showcase.rs` does not exist yet.
