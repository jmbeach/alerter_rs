---
id: ar-9x4w
status: closed
deps: []
links: []
created: 2026-03-08T00:30:00Z
type: task
priority: 2
parent: ar-erxo
tags: [tdd]
---
# Test: Example binary compiles and argument parsing works

Test that the showcase example binary compiles and accepts notification type arguments. File: tests/showcase_compile_test.rs

## Acceptance Criteria

Write tests that FAIL verifying:
1. `cargo build --example showcase` succeeds (the example file exists and compiles).
2. Running the showcase binary with argument `basic` exits successfully.
3. Running the showcase binary with argument `sound` exits successfully.
4. Running the showcase binary with argument `subtitle` exits successfully.

Tests must fail since `examples/showcase.rs` does not exist yet.
