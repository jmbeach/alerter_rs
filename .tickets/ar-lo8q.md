---
id: ar-lo8q
status: closed
deps: []
links: []
created: 2026-03-08T00:35:00Z
type: task
priority: 2
parent: ar-3atw
tags: [impl]
---
# Implement integration-tests.sh

Create `integration-tests.sh` shell script that builds and runs the showcase example.

## Files
- integration-tests.sh (create, must be executable)

## Acceptance Criteria

1. Starts with `#!/usr/bin/env bash` shebang line.
2. Marked executable (chmod +x).
3. Runs `cargo build --example showcase` and exits on failure.
4. Launches notification types concurrently using `&` background processes.
5. Uses `wait` to collect results and tracks pass/fail counts.
6. Prints a summary with pass/fail counts.
7. Exits with code 0 if all pass, code 1 if any fail.
8. All tests in tests/integration_script_test.rs and tests/integration_script_content_test.rs pass.

## Reference
- Spec: openspec/changes/example-binary-and-integration-tests/specs/integration-test-script/spec.md
