---
id: ar-v23l
status: closed
deps: []
links: []
created: 2026-03-08T00:30:00Z
type: task
priority: 2
parent: ar-erxo
tags: [tdd]
---
# Test: Example binary uses only public alerter_rs API

Test that the showcase example only imports public types from alerter_rs. File: tests/showcase_api_test.rs

## Acceptance Criteria

Write tests that FAIL verifying:
1. The source file `examples/showcase.rs` exists.
2. The source file imports `alerter_rs::Alerter` (and optionally `alerter_rs::AlerterResponse`).
3. The source file does NOT import any private modules (no `alerter_rs::binary`, `alerter_rs::error` internal paths beyond public re-exports).

Tests must fail since `examples/showcase.rs` does not exist yet.
