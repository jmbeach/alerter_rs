# notification-sending Specification

## Purpose
TBD - created by archiving change alerter-swift-wrapper. Update Purpose after archive.
## Requirements
### Requirement: Send a notification with a message
The library SHALL provide a method to send a macOS notification with a required message body.

#### Scenario: Basic notification
- **WHEN** the user calls `Alerter::new("Hello world").send()`
- **THEN** a macOS notification is displayed with "Hello world" as the message body

### Requirement: Configure notification title
The library SHALL allow setting a custom notification title via `.title()`.

#### Scenario: Custom title
- **WHEN** the user calls `Alerter::new("msg").title("My App").send()`
- **THEN** the notification displays "My App" as the title

### Requirement: Configure notification subtitle
The library SHALL allow setting a subtitle via `.subtitle()`.

#### Scenario: Subtitle display
- **WHEN** the user calls `Alerter::new("msg").subtitle("Important").send()`
- **THEN** the notification displays "Important" as the subtitle

### Requirement: Play a sound with notification
The library SHALL allow specifying a sound via `.sound()`. The value "default" SHALL play the system default sound.

#### Scenario: Default sound
- **WHEN** the user calls `Alerter::new("msg").sound("default").send()`
- **THEN** the notification plays the default macOS notification sound

### Requirement: Configure action buttons
The library SHALL allow specifying action button labels via `.actions()` and an optional dropdown label via `.dropdown_label()`.

#### Scenario: Single action
- **WHEN** the user calls `Alerter::new("msg").actions(vec!["OK"]).send()`
- **THEN** the notification displays an "OK" action button

#### Scenario: Multiple actions with dropdown
- **WHEN** the user calls `Alerter::new("msg").actions(vec!["Yes", "No", "Maybe"]).dropdown_label("Choose").send()`
- **THEN** the notification displays a dropdown labeled "Choose" with options "Yes", "No", "Maybe"

### Requirement: Configure reply-type notification
The library SHALL allow displaying a reply-type alert via `.reply()` with placeholder text.

#### Scenario: Reply notification
- **WHEN** the user calls `Alerter::new("msg").reply("Type here...").send()`
- **THEN** the notification displays a text input field with placeholder "Type here..."

### Requirement: Configure close button label
The library SHALL allow customizing the close button label via `.close_label()`.

#### Scenario: Custom close label
- **WHEN** the user calls `Alerter::new("msg").close_label("Dismiss").send()`
- **THEN** the notification close button shows "Dismiss"

### Requirement: Configure notification group
The library SHALL allow setting a group ID via `.group()` for notification replacement.

#### Scenario: Group replacement
- **WHEN** the user sends two notifications with `.group("updates")`
- **THEN** the second notification replaces the first in the notification center

### Requirement: Configure sender identity
The library SHALL allow impersonating another app via `.sender()` with a bundle identifier.

#### Scenario: Custom sender
- **WHEN** the user calls `Alerter::new("msg").sender("com.apple.Safari").send()`
- **THEN** the notification appears as if sent by Safari

### Requirement: Configure app icon
The library SHALL allow setting a custom app icon via `.app_icon()` with a URL or file path.

#### Scenario: Custom icon
- **WHEN** the user calls `Alerter::new("msg").app_icon("/path/to/icon.png").send()`
- **THEN** the notification displays the specified image as its app icon

### Requirement: Configure content image
The library SHALL allow attaching an image via `.content_image()` with a URL or file path.

#### Scenario: Content image
- **WHEN** the user calls `Alerter::new("msg").content_image("/path/to/photo.png").send()`
- **THEN** the notification displays the specified image as attached content

### Requirement: Configure notification timeout
The library SHALL allow auto-closing the notification after N seconds via `.timeout()`.

#### Scenario: Auto-close
- **WHEN** the user calls `Alerter::new("msg").timeout(10).send()`
- **THEN** the notification auto-closes after 10 seconds if not interacted with

### Requirement: Request JSON output
The library SHALL allow requesting JSON output via `.json(true)` and parse the result into a typed struct.

#### Scenario: JSON response
- **WHEN** the user calls `Alerter::new("msg").json(true).send()`
- **THEN** the method returns a parsed `AlerterResponse` with structured fields

### Requirement: Configure notification delay
The library SHALL allow delaying notification delivery via `.delay()` (seconds) or `.at()` (specific time).

#### Scenario: Delayed delivery
- **WHEN** the user calls `Alerter::new("msg").delay(30).send()`
- **THEN** the notification is delivered after 30 seconds

#### Scenario: Scheduled delivery
- **WHEN** the user calls `Alerter::new("msg").at("14:30").send()`
- **THEN** the notification is delivered at 14:30

### Requirement: Ignore Do Not Disturb
The library SHALL allow sending notifications even when DND is enabled via `.ignore_dnd(true)`.

#### Scenario: DND override
- **WHEN** the user calls `Alerter::new("msg").ignore_dnd(true).send()` while DND is active
- **THEN** the notification is still delivered

### Requirement: Return notification interaction result
The `send()` method SHALL return a `Result<AlerterResponse, AlerterError>` that captures the user's interaction (button clicked, reply text, timeout, etc.).

#### Scenario: User clicks action
- **WHEN** the user interacts with a notification by clicking an action button
- **THEN** `send()` returns an `AlerterResponse` indicating which action was selected

