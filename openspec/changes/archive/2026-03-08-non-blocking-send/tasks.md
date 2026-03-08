## 1. NotificationHandle type

- [x] 1.1 Create `src/handle.rs` with `NotificationHandle` struct wrapping `Option<Child>` and `json: bool`
- [x] 1.2 Implement `wait()` — take stdout/stderr from child, call `child.wait()`, parse response (JSON or plain text)
- [x] 1.3 Implement `try_wait()` — call `child.try_wait()`, return `Ok(None)` if still running, parse response if exited
- [x] 1.4 Implement `detach()` — consume self, forget the child process without killing it
- [x] 1.5 Implement `Drop` — kill the child process if still running

## 2. Alerter integration

- [x] 2.1 Add `send_async()` method to `Alerter` using `Command::spawn()` with piped stdout/stderr, returning `NotificationHandle`
- [x] 2.2 Export `NotificationHandle` from `src/lib.rs`

## 3. Documentation

- [x] 3.1 Create a "Working with Async" guide documenting usage patterns: fire-and-forget, deferred wait, and the Vec + try_wait() reaping pattern for managing multiple concurrent notifications

## 4. Tests

- [x] 4.1 Add unit test verifying `send_async()` returns immediately without blocking
- [x] 4.2 Add test for `wait()` returning a valid `AlerterResponse`
- [x] 4.3 Add test for `try_wait()` returning `None` before interaction and `Some` after
- [x] 4.4 Add test for `detach()` consuming the handle without killing the process
