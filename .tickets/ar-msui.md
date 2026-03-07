---
id: ar-msui
status: closed
deps: []
links: []
created: 2026-03-07T22:35:34Z
type: task
priority: 2
parent: ar-qkjd
tags: [impl]
---
# Wire up lib.rs public API re-exports

Update src/lib.rs to declare modules and re-export public types. Files: src/lib.rs

## Acceptance Criteria

1. alerter_rs exports Alerter, AlerterResponse, AlerterError, and binary module. 2. All tests in tests/api_surface_tests.rs pass.

