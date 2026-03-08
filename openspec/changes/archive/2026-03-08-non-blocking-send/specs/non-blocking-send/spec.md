## ADDED Requirements

### Requirement: Non-blocking notification send
The `Alerter` struct SHALL provide a `send_async()` method that spawns the alerter subprocess and returns immediately with a `NotificationHandle`, without blocking the calling thread.

#### Scenario: Fire and forget notification
- **WHEN** the caller invokes `send_async()` on a configured `Alerter`
- **THEN** the method SHALL return a `NotificationHandle` immediately while the notification is displayed to the user in the background

#### Scenario: Builder configuration preserved
- **WHEN** the caller configures actions, reply, timeout, json, or any other builder option before calling `send_async()`
- **THEN** all configured options SHALL be passed to the alerter subprocess identically to how `send()` passes them

### Requirement: NotificationHandle wait
The `NotificationHandle` SHALL provide a `wait()` method that blocks until the notification subprocess exits and returns the parsed `AlerterResponse`.

#### Scenario: Blocking wait for response
- **WHEN** the caller invokes `wait()` on a `NotificationHandle`
- **THEN** the method SHALL block until the user interacts with the notification and return `Result<AlerterResponse, AlerterError>`

#### Scenario: Response parsing matches send
- **WHEN** the notification was configured with `.json(true)` and the user clicks an action
- **THEN** `wait()` SHALL parse the JSON response identically to how `send()` parses it

### Requirement: NotificationHandle try_wait
The `NotificationHandle` SHALL provide a `try_wait()` method that checks if the subprocess has exited without blocking.

#### Scenario: Notification not yet responded to
- **WHEN** the caller invokes `try_wait()` and the user has not yet interacted with the notification
- **THEN** the method SHALL return `Ok(None)` without blocking

#### Scenario: Notification responded to
- **WHEN** the caller invokes `try_wait()` and the user has already interacted with the notification
- **THEN** the method SHALL return `Ok(Some(AlerterResponse))` with the parsed response

### Requirement: NotificationHandle drop behavior
When a `NotificationHandle` is dropped, the notification subprocess SHALL be killed to prevent orphaned processes.

#### Scenario: Handle dropped without waiting
- **WHEN** a `NotificationHandle` is dropped without calling `wait()` or `try_wait()`
- **THEN** the alerter subprocess SHALL be terminated

### Requirement: NotificationHandle detach
The `NotificationHandle` SHALL provide a `detach()` method that consumes the handle and lets the subprocess continue running independently.

#### Scenario: Detached notification persists
- **WHEN** the caller invokes `detach()` on a `NotificationHandle`
- **THEN** the notification subprocess SHALL continue running and the handle is consumed without killing the process
