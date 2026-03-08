---
id: ar-u56w
status: closed
deps: []
links: []
created: 2026-03-08T00:30:00Z
type: task
priority: 2
parent: ar-erxo
tags: [tdd]
---
# Test: Example binary handles all notification type arguments

Test that the showcase binary handles each notification type argument correctly. File: tests/showcase_notifications_test.rs

## Acceptance Criteria

Write tests that FAIL verifying the showcase binary runs without error for each of these arguments:
1. `actions` - notification with action buttons
2. `dropdown` - notification with dropdown
3. `reply` - notification with reply field
4. `icon` - notification with app icon
5. `content-image` - notification with content image
6. `group` - notification with group ID
7. `sender` - notification with custom sender
8. `json` - notification with JSON output
9. `close-label` - notification with custom close label
10. `ignore-dnd` - notification with ignore-dnd enabled
11. `remove` - remove group notifications

Tests must fail since `examples/showcase.rs` does not exist yet.
