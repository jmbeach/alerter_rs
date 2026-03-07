## 1. Build System

- [x] 1.1 Create `build.rs` that runs `swift build -c release` on the `alerter/` directory and outputs the binary path via `cargo:rustc-env`
- [x] 1.2 Embed the compiled alerter binary into the library using `include_bytes!` with the path from `build.rs`
- [x] 1.3 Update `Cargo.toml` with required dependencies (`serde`, `serde_json`, `dirs`) and metadata (macOS-only target, build script)

## 2. Binary Extraction

- [x] 2.1 Implement binary cache module (`src/binary.rs`): extract embedded binary to `~/.cache/alerter_rs/`, set executable permission, and reuse on subsequent calls
- [x] 2.2 Add cache invalidation using a hash of the embedded binary to detect version changes

## 3. Core API — Notification Sending

- [x] 3.1 Define `Alerter` builder struct with fields for all CLI options (message, title, subtitle, sound, actions, dropdown_label, reply, close_label, group, sender, app_icon, content_image, timeout, json, delay, at, ignore_dnd)
- [x] 3.2 Implement builder methods (each returning `&mut Self`) for all options
- [x] 3.3 Implement `send()` method that builds a `Command` with the appropriate CLI arguments, invokes the cached binary, and captures stdout/stderr
- [x] 3.4 Define `AlerterResponse` struct and parse the alerter output (both plain text and JSON modes)
- [x] 3.5 Define `AlerterError` enum covering build/extraction failures, process spawn failures, and alerter runtime errors

## 4. Core API — Notification Management

- [x] 4.1 Implement `Alerter::remove(group_id)` static method that invokes `alerter --remove <group_id>`
- [x] 4.2 Implement `Alerter::list(group_id)` static method that invokes `alerter --list <group_id>`

## 5. Public API Surface

- [x] 5.1 Wire up `src/lib.rs` to re-export `Alerter`, `AlerterResponse`, `AlerterError`, and any supporting types
- [x] 5.2 Add basic integration test that verifies the binary can be extracted and `--help` can be invoked successfully

## 6. Documentation

- [ ] 6.1 Create README.md with project overview, usage examples, API reference, build requirements (Swift toolchain), and macOS version compatibility
