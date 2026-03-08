## Context

alerter_rs is a Rust library wrapping the macOS `alerter` binary. It provides a builder API (`Alerter::new()`) with methods for title, subtitle, sound, actions, icons, timeouts, etc. Currently there are unit tests but no runnable example showing real notifications.

## Goals / Non-Goals

**Goals:**
- Provide a single example binary that demonstrates every builder method
- Create a shell script that builds and runs the example to validate real notification delivery
- Use short timeouts so notifications auto-dismiss during testing

**Non-Goals:**
- Interactive test validation (tests are fire-and-forget, not checking user clicks)
- CI integration (notifications require a macOS GUI session)
- Modifying library code

## Decisions

1. **Single example binary vs multiple**: Use one `examples/showcase.rs` with a subcommand/argument per notification type. This keeps it simple — one file to read, one binary to build. The shell script calls it with different arguments.

2. **Argument-driven example**: The binary accepts a notification type name (e.g., `sound`, `actions`, `icon`) as a CLI argument and sends the corresponding notification. This lets the shell script run them in parallel with `&` and `wait`.

3. **Short timeouts with one interactive**: Most notifications use a 5-second timeout so they auto-dismiss. However, at least one notification (e.g., the actions test) SHALL have no timeout, requiring the user to click a button. This validates interactive response handling.

4. **No external CLI parsing crate**: Use simple `std::env::args()` matching since we only need a single positional argument. Avoids adding dependencies for a demo.

## Risks / Trade-offs

- [Notifications require macOS GUI session] → Document that tests must run on a Mac with a display
- [Icon paths may not exist on all systems] → Use `/Applications/Safari.app/Contents/Resources/AppIcon.icns` which exists on all Macs
- [Parallel notifications may overwhelm Notification Center] → Acceptable for a demo; add small delays between launches if needed
