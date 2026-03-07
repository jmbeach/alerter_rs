## Context

This is a new Rust crate (`alerter_rs`) that wraps the [vjeantet/alerter](https://github.com/vjeantet/alerter) Swift CLI tool. The alerter source is vendored at `./alerter/` and builds via Swift Package Manager. The crate targets macOS only.

The alerter binary is a standalone executable that uses macOS `UserNotifications` framework to send rich notifications with actions, replies, icons, sounds, scheduling, and group management.

## Goals / Non-Goals

**Goals:**
- Compile the vendored alerter Swift project at `cargo build` time via `build.rs`
- Embed the compiled binary into the Rust library (via `include_bytes!`)
- At runtime, extract the binary to a cache directory and invoke it as a subprocess
- Provide a typed Rust API (builder pattern) covering all alerter CLI options
- Parse alerter's stdout/stderr into typed Rust response structs
- Support JSON output mode for structured responses

**Non-Goals:**
- FFI bindings to Swift code (too complex; subprocess invocation is simpler and alerter is designed as a CLI)
- Cross-platform support (alerter is macOS-only by nature)
- Async/tokio integration (can be added later; sync subprocess calls are sufficient)
- Modifying the vendored alerter source code

## Decisions

### 1. Binary embedding via `include_bytes!` over runtime PATH lookup

**Choice**: Embed the compiled alerter binary into the Rust library using `include_bytes!` in `build.rs`, then extract to `~/.cache/alerter_rs/` at runtime.

**Alternatives considered**:
- *Assume alerter is installed*: Defeats the purpose — users shouldn't need to install anything.
- *Download at build time*: Requires network access during builds; vendored source is more reliable.
- *Ship as a separate binary artifact*: More complex distribution; single crate is simpler.

**Rationale**: `include_bytes!` ensures the binary is always available. The cache directory avoids re-extracting on every call. A version/hash check ensures the cached binary stays in sync with the crate version.

### 2. Subprocess invocation over Swift FFI

**Choice**: Invoke the alerter binary as a subprocess via `std::process::Command`.

**Alternatives considered**:
- *Swift-to-C FFI bridge*: Would require restructuring alerter's Swift code, maintaining C headers, and dealing with Swift runtime linking complexity.
- *Rewrite notification logic in Rust*: Large effort, would need to use objc2 crate for Objective-C runtime bindings.

**Rationale**: alerter is designed as a CLI tool. Subprocess invocation is simple, reliable, and means we can track upstream changes without modification.

### 3. Builder pattern API

**Choice**: Use a builder pattern (`Alerter::new("message").title("Hello").sound("default").send()`) for constructing notifications.

**Rationale**: Maps naturally to the CLI's optional flags. Provides good ergonomics and compile-time safety.

### 4. Build-time Swift compilation via `build.rs`

**Choice**: `build.rs` runs `swift build -c release` on the vendored `alerter/` directory.

**Rationale**: Swift Package Manager handles dependency resolution (swift-argument-parser) and compilation. The build script captures the output binary path and makes it available via `include_bytes!`.

### 5. Crate version mirrors alerter version

**Choice**: The crate's SemVer version directly matches the bundled alerter version. Rust-only fixes use patch bumps.

**Alternatives considered**:
- *Independent versioning with build metadata*: e.g., `0.1.0+alerter-1.2.0`. More flexible but less transparent.
- *Version constant only*: Crate has its own version, exposes `Alerter::alerter_version()`. Harder for users to know at a glance.

**Rationale**: This is the standard approach for wrapper crates. Users immediately know which alerter version they're getting from the crate version alone.

## Risks / Trade-offs

- **[Risk] Swift toolchain required at build time** → Document this requirement clearly. Swift is pre-installed on macOS with Xcode Command Line Tools, which most developers have.
- **[Risk] Binary size increase** → The alerter binary is small (~2-3MB). Acceptable for the convenience.
- **[Risk] macOS version compatibility** → alerter requires macOS 13+. Match this in crate documentation.
- **[Risk] Cache directory permissions/cleanup** → Use `dirs::cache_dir()` for platform-appropriate cache location. Include binary hash in filename to handle version updates.
- **[Trade-off] Subprocess overhead** → Each notification requires spawning a process. Acceptable for notification use cases (infrequent, user-facing).
