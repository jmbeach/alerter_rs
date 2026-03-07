---
id: ar-6upb
status: closed
deps: []
links: []
created: 2026-03-07T22:31:41Z
type: task
priority: 2
parent: ar-635n
tags: [tdd]
---
# Test: binary embedding and extraction

Test the binary-bundling capability: build.rs embeds alerter binary, runtime extracts to cache dir, reuses cached binary, invalidates on version change. File: tests/binary_tests.rs

## Acceptance Criteria

Write tests that FAIL verifying: 1) The embedded binary constant exists and is non-empty, 2) Binary extraction to cache dir creates an executable file, 3) Cache reuse on subsequent calls (no re-extraction), 4) Cache invalidation when binary hash changes. Tests must fail since no implementation exists yet.

