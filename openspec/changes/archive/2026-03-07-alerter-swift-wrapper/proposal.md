## Why

The `alerter` program (by vjeantet) provides a powerful CLI for sending macOS user notifications with rich options (actions, replies, custom icons, sounds, scheduling, etc.). Currently, Rust projects that want to send macOS notifications must either shell out to a pre-installed binary or use limited native Rust notification crates. By wrapping alerter as a Rust library with the binary bundled at build time, users get full macOS notification capabilities as a simple `cargo` dependency — no external installation required.

## What Changes

- Add a `build.rs` that compiles the vendored alerter Swift project into a binary and embeds it into the Rust library.
- Expose a Rust API (`Alerter` struct with builder pattern) covering all alerter CLI options: message, title, subtitle, sound, actions, reply, icons, scheduling, group management, and more.
- The library extracts/invokes the bundled alerter binary at runtime, translating Rust API calls into CLI invocations and parsing the output (including JSON mode).
- Provide typed response structs for notification results (which button was clicked, reply text, etc.).

## Capabilities

### New Capabilities
- `notification-sending`: Builder-based API for constructing and sending macOS notifications with all alerter options (message, title, subtitle, sound, actions, reply, icons, timeouts, scheduling, DND override, sender impersonation).
- `notification-management`: API for listing and removing notifications by group ID.
- `binary-bundling`: Build-time compilation and runtime extraction of the vendored alerter Swift binary so users need no external installation.

### Modified Capabilities

(none — this is a new library)

## Impact

- **Dependencies**: Requires Swift toolchain available at build time (standard on macOS with Xcode/CLT installed).
- **Platform**: macOS-only library. Build will fail on non-macOS targets.
- **Build**: `build.rs` will invoke `swift build` on the vendored `alerter/` directory during `cargo build`.
- **Runtime**: The compiled alerter binary is embedded in the library and extracted to a temp/cache location on first use.
- **API surface**: New public types — `Alerter` (builder), `AlerterResponse`, error types.
- **Versioning**: The crate version directly mirrors the bundled alerter version (e.g., alerter v1.2.0 → crate `1.2.0`). Rust-only fixes use patch bumps.
