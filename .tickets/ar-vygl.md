---
id: ar-vygl
status: closed
deps: []
links: []
created: 2026-03-07T22:35:34Z
type: task
priority: 2
parent: ar-qkjd
tags: [impl]
---
# Implement build.rs and binary embedding

Create build.rs that runs swift build on alerter/, embeds the binary. Create src/binary.rs with ALERTER_BINARY constant. Update Cargo.toml. Files: build.rs, Cargo.toml, src/binary.rs

## Acceptance Criteria

1. build.rs compiles alerter/ via 'swift build -c release' and exposes the binary path. 2. The compiled binary is embedded via include_bytes\! as ALERTER_BINARY constant. 3. Cargo.toml has all required dependencies (serde, serde_json, dirs). 4. Tests in tests/binary_tests.rs pass (embedded_binary_constant_exists_and_is_non_empty, embedded_binary_starts_with_macho_magic).

