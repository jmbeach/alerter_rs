## ADDED Requirements

### Requirement: Example binary demonstrates all notification options
The example binary SHALL accept a notification type as a CLI argument and send the corresponding notification using the alerter_rs library. It SHALL demonstrate the following notification types: basic, sound, actions, dropdown, reply, icon, content-image, subtitle, group, timeout, sender, close-label, ignore-dnd, json, and remove.

#### Scenario: Basic notification
- **WHEN** the binary is run with argument `basic`
- **THEN** a notification is sent with only a message and title, with a 5-second timeout

#### Scenario: Notification with sound
- **WHEN** the binary is run with argument `sound`
- **THEN** a notification is sent with sound set to "default" and a 5-second timeout

#### Scenario: Notification with action buttons
- **WHEN** the binary is run with argument `actions`
- **THEN** a notification is sent with actions "Yes" and "No" and a close label "Maybe" with no timeout, requiring the user to click

#### Scenario: Notification with dropdown
- **WHEN** the binary is run with argument `dropdown`
- **THEN** a notification is sent with a dropdown label and multiple action items and a 5-second timeout

#### Scenario: Notification with reply field
- **WHEN** the binary is run with argument `reply`
- **THEN** a notification is sent with a reply placeholder text and a 5-second timeout

#### Scenario: Notification with app icon
- **WHEN** the binary is run with argument `icon`
- **THEN** a notification is sent with an app icon path pointing to a system icon and a 5-second timeout

#### Scenario: Notification with content image
- **WHEN** the binary is run with argument `content-image`
- **THEN** a notification is sent with a content image path and a 5-second timeout

#### Scenario: Notification with subtitle
- **WHEN** the binary is run with argument `subtitle`
- **THEN** a notification is sent with both title and subtitle set and a 5-second timeout

#### Scenario: Notification with group
- **WHEN** the binary is run with argument `group`
- **THEN** a notification is sent with a group identifier and a 5-second timeout

#### Scenario: Notification with custom sender
- **WHEN** the binary is run with argument `sender`
- **THEN** a notification is sent with sender set to "com.apple.Safari" and a 5-second timeout

#### Scenario: Notification with JSON output
- **WHEN** the binary is run with argument `json`
- **THEN** a notification is sent with JSON mode enabled and the response is printed as JSON

#### Scenario: Notification with close label
- **WHEN** the binary is run with argument `close-label`
- **THEN** a notification is sent with a custom close label and a 5-second timeout

#### Scenario: Notification with ignore-dnd
- **WHEN** the binary is run with argument `ignore-dnd`
- **THEN** a notification is sent with ignore_dnd enabled and a 5-second timeout

#### Scenario: Remove group notifications
- **WHEN** the binary is run with argument `remove`
- **THEN** the binary calls `Alerter::remove` for a test group ID

#### Scenario: Unknown argument
- **WHEN** the binary is run with an unrecognized argument
- **THEN** the binary prints a usage message listing all available notification types and exits with a non-zero status

### Requirement: Example binary uses only the public alerter_rs API
The example binary SHALL import and use only `alerter_rs::Alerter` and `alerter_rs::AlerterResponse` from the library. It SHALL NOT access any private modules.

#### Scenario: Public API usage
- **WHEN** the example source code is reviewed
- **THEN** only `alerter_rs::Alerter` and related public types are imported
