## Context

`Alerter::send()` currently uses `Command::output()` which blocks until the alerter subprocess exits. The subprocess only exits after user interaction (click, dismiss, reply) or timeout. This means any program using actions or reply must either block its main thread or manually spawn a `std::thread` to avoid blocking.

The library has zero async or threading dependencies today — only `serde`, `serde_json`, `dirs`, and `sha2`.

## Goals / Non-Goals

**Goals:**
- Provide a `send_async()` method that returns immediately after spawning the notification subprocess
- Return a `NotificationHandle` that lets callers check for or wait on the response later
- Keep the existing `send()` method unchanged
- No new external dependencies — use only `std::process::Child` and `std::io`

**Non-Goals:**
- Tokio/async-std integration or `Future` implementation (users can wrap in `spawn_blocking` themselves)
- Callback-based API (closures invoked on notification response)
- Thread pool or background worker management

## Decisions

### Use `Command::spawn()` instead of `Command::output()`

The non-blocking path will use `Command::spawn()` which returns a `Child` immediately. The `NotificationHandle` wraps this `Child` and provides `wait()` (blocking) and `try_wait()` (polling) to retrieve the response.

**Alternative considered**: Spawning a `std::thread` internally that calls `Command::output()` and sends the result over a channel. This adds complexity (channels, thread management) for no benefit since `Child` already provides the exact semantics we need.

### `NotificationHandle` owns the child process

The handle stores the `Child`, the `json` flag (needed for response parsing), and stdout/stderr pipes. When the user calls `wait()` or `try_wait()`, the handle reads stdout and parses the response the same way `send()` does today.

**Alternative considered**: Returning `std::process::Child` directly. This leaks implementation details and forces users to handle response parsing themselves.

### Pipe stdout/stderr explicitly

We need to set `stdout(Stdio::piped())` and `stderr(Stdio::piped())` on the `Command` before spawning so we can capture the output later. Without piped stdio, the output goes to the parent process's stdout/stderr and is lost.

## Risks / Trade-offs

- **Dropped handles**: If a `NotificationHandle` is dropped without calling `wait()`, the child process becomes orphaned. The alerter binary will still show the notification and eventually exit, but the response is lost. This is acceptable for fire-and-forget use cases — it's the whole point. We'll implement `Drop` to kill the child process to avoid orphans.
  → Mitigation: Document that dropping the handle kills the notification subprocess. Users who want true fire-and-forget (notification persists after drop) can call a `.detach()` method that consumes the handle without killing the process.

- **No automatic JSON mode**: `send_async()` inherits whatever `json` setting the builder has. If the user doesn't set `.json(true)`, `try_wait()` returns a plain-text parsed response, same as `send()` today. No special handling needed.
