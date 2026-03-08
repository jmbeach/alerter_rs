## Why

The library has no runnable example showing how to use the full alerter_rs API. Users need a concrete reference to understand how to construct notifications with different options (sound, actions, icons, timeouts, etc.). An integration test script would also validate that the library works end-to-end on a real system.

## What Changes

- Add a Cargo example binary (`examples/showcase.rs`) that demonstrates every `Alerter` builder method by sending multiple notifications
- Add an `integration-tests.sh` shell script that builds and runs the example binary, exercising all major notification features in parallel

## Capabilities

### New Capabilities
- `example-binary`: A Rust example binary that exercises the full alerter_rs API surface (all builder methods, send, remove, list)
- `integration-test-script`: A shell script that builds the example and runs it to test all major notification types

### Modified Capabilities

## Impact

- New files: `examples/showcase.rs`, `integration-tests.sh`
- No changes to library code
- Cargo.toml may need `[[example]]` entry if the default convention doesn't pick it up automatically
