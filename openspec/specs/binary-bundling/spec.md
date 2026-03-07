# binary-bundling Specification

## Purpose
TBD - created by archiving change alerter-swift-wrapper. Update Purpose after archive.
## Requirements
### Requirement: Compile alerter at build time
The `build.rs` script SHALL compile the vendored alerter Swift project using `swift build -c release` during `cargo build`.

#### Scenario: Successful build
- **WHEN** a user runs `cargo build` with Swift toolchain installed
- **THEN** the alerter binary is compiled from `alerter/` and embedded into the crate

#### Scenario: Missing Swift toolchain
- **WHEN** a user runs `cargo build` without Swift toolchain
- **THEN** the build fails with a clear error message indicating Swift is required

### Requirement: Embed binary into library
The compiled alerter binary SHALL be embedded into the Rust library via `include_bytes!` so no external files are needed at runtime.

#### Scenario: Self-contained library
- **WHEN** a user adds `alerter_rs` as a dependency and builds their project
- **THEN** the resulting binary contains the alerter executable with no additional files to distribute

### Requirement: Extract binary at runtime
The library SHALL extract the embedded binary to a cache directory on first use and reuse it for subsequent calls.

#### Scenario: First invocation
- **WHEN** `send()` is called for the first time
- **THEN** the embedded binary is extracted to a cache directory and made executable

#### Scenario: Subsequent invocations
- **WHEN** `send()` is called after the binary has been extracted
- **THEN** the cached binary is reused without re-extraction

### Requirement: Cache invalidation on version change
The library SHALL detect when the embedded binary differs from the cached version and re-extract it.

#### Scenario: Crate update
- **WHEN** the user updates `alerter_rs` to a new version with a different embedded binary
- **THEN** the library extracts the new binary, replacing the old cached one

