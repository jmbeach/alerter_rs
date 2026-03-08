---
id: ar-fyd4
status: closed
deps: []
links: []
created: 2026-03-08T00:35:00Z
type: task
priority: 2
parent: ar-3atw
tags: [impl]
---
# Implement examples/showcase.rs

Create `examples/showcase.rs` that demonstrates the full alerter_rs API.

## Files
- examples/showcase.rs (create)

## Acceptance Criteria

1. The binary accepts a notification type as a CLI argument via `std::env::args()`.
2. Supported types: basic, sound, actions, dropdown, reply, icon, content-image, subtitle, group, sender, json, close-label, ignore-dnd, remove.
3. Each type sends the appropriate notification using the `alerter_rs::Alerter` builder API.
4. Most notifications use a 5-second timeout; actions has no timeout.
5. Unknown arguments or no arguments print a usage message listing all types and exit with non-zero status.
6. Only imports public API types: `alerter_rs::Alerter`, `alerter_rs::AlerterResponse`.
7. All tests in tests/showcase_compile_test.rs, tests/showcase_notifications_test.rs, tests/showcase_usage_test.rs, and tests/showcase_api_test.rs pass.

## Reference
- Spec: openspec/changes/example-binary-and-integration-tests/specs/example-binary/spec.md
- Design: openspec/changes/example-binary-and-integration-tests/design.md
