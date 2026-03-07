---
id: ar-lyth
status: closed
deps: []
links: []
created: 2026-03-07T22:31:54Z
type: task
priority: 2
parent: ar-635n
tags: [tdd]
---
# Test: Alerter builder construction and argument generation

Test the notification-sending builder API: struct creation, method chaining, and CLI argument generation. File: tests/builder_tests.rs

## Acceptance Criteria

Write tests that FAIL verifying: 1) Alerter::new(msg) creates builder with message set, 2) All builder methods (.title(), .subtitle(), .sound(), .actions(), .dropdown_label(), .reply(), .close_label(), .group(), .sender(), .app_icon(), .content_image(), .timeout(), .json(), .delay(), .at(), .ignore_dnd()) chain correctly and store values, 3) The builder generates correct CLI arguments from its state. Tests must fail since no implementation exists yet.

