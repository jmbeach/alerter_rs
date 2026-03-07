---
id: ar-u9w2
status: closed
deps: [ar-msui]
links: []
created: 2026-03-07T22:35:34Z
type: task
priority: 2
parent: ar-qkjd
tags: [impl]
---
# Implement Alerter builder struct and build_args()

Define Alerter struct with all option fields and builder methods. Implement build_args() to generate CLI arguments. Files: src/alerter.rs

## Acceptance Criteria

1. Alerter::new(msg) creates builder. 2. All builder methods chain: title(), subtitle(), sound(), actions(), dropdown_label(), reply(), close_label(), group(), sender(), app_icon(), content_image(), timeout(), json(), delay(), at(), ignore_dnd(). 3. build_args() returns correct Vec<String> with -- prefixed kebab-case flags. 4. All tests in tests/builder_tests.rs pass.

