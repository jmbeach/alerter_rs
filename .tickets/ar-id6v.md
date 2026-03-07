---
id: ar-id6v
status: closed
deps: [ar-msui, ar-u9w2, ar-13s6]
links: []
created: 2026-03-07T22:35:34Z
type: task
priority: 2
parent: ar-qkjd
tags: [impl]
---
# Implement AlerterResponse, AlerterError, and send()

Define AlerterResponse and AlerterError types. Implement send() on Alerter. Files: src/alerter.rs, src/error.rs, src/response.rs

## Acceptance Criteria

1. AlerterResponse has activation_type and activation_value fields, from_plain_text() and from_json() constructors, implements Debug. 2. AlerterError has BinaryExtraction, ProcessSpawn, Runtime variants, implements std::error::Error + Display + Debug. 3. send() invokes the cached binary with build_args(), parses output. 4. All tests in tests/send_tests.rs and tests/api_surface_tests.rs pass.

