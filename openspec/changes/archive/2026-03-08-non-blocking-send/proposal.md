## Why

The current `Alerter::send()` method blocks the calling thread until the user interacts with the notification (clicks an action, dismisses it, or it times out). Programs that want to show a notification with click actions and continue executing must manually spawn threads. The library should provide a built-in non-blocking send option so callers can fire off a notification and optionally check its result later.

## What Changes

- Add a `send_async()` method to `Alerter` that spawns the alerter subprocess without waiting, returning a `NotificationHandle`
- Add a `NotificationHandle` type that wraps the child process and provides:
  - `wait()` — blocks until the notification completes and returns the response
  - `try_wait()` — non-blocking check if the notification has been responded to
- Use `Command::spawn()` instead of `Command::output()` internally for the non-blocking path

## Capabilities

### New Capabilities
- `non-blocking-send`: Non-blocking notification delivery via `send_async()` returning a handle for deferred response collection

### Modified Capabilities

## Impact

- **Code**: `src/alerter.rs` (new method), new `src/handle.rs` module for `NotificationHandle`
- **Public API**: Two new public types/methods added; no breaking changes to existing API
- **Dependencies**: None — uses only `std::process::Child` from the standard library
