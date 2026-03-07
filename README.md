# alerter_rs

A Rust wrapper around [vjeantet/alerter](https://github.com/vjeantet/alerter) — send rich macOS user notifications from Rust with zero external dependencies at runtime.

The alerter binary is compiled from source at build time and embedded directly into your Rust binary. Users of your application never need to install alerter separately.

## Requirements

- **macOS 13+**
- **Swift toolchain** (included with Xcode Command Line Tools)
- **Rust 2024 edition**

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alerter_rs = { git = "https://github.com/jmbeach/alerter_rs.git" }
```

## Usage

### Send a notification

```rust
use alerter_rs::Alerter;

let response = Alerter::new("Hello from Rust!")
    .title("My App")
    .subtitle("Important update")
    .sound("default")
    .send()?;

println!("User clicked: {}", response.activation_type);
```

### Actions and dropdowns

```rust
let response = Alerter::new("Choose an option")
    .actions(vec!["Yes", "No", "Maybe"])
    .dropdown_label("Options")
    .send()?;
```

### Reply notifications

```rust
let response = Alerter::new("What do you think?")
    .reply("Type your reply...")
    .send()?;

if let Some(reply_text) = response.activation_value {
    println!("User replied: {reply_text}");
}
```

### JSON output

```rust
let response = Alerter::new("Click me")
    .json(true)
    .send()?;
```

### Scheduled delivery

```rust
// Deliver after 30 seconds
Alerter::new("Reminder").delay(30).send()?;

// Deliver at a specific time
Alerter::new("Meeting soon").at("14:30").send()?;
```

### Custom appearance

```rust
Alerter::new("Update available")
    .title("My App")
    .sender("com.apple.Safari")       // appear as Safari
    .app_icon("/path/to/icon.png")
    .content_image("/path/to/img.png")
    .close_label("Dismiss")
    .timeout(10)                       // auto-close after 10s
    .ignore_dnd(true)                  // bypass Do Not Disturb
    .group("updates")                  // replace previous in group
    .send()?;
```

### Manage notifications

```rust
use alerter_rs::Alerter;

// Remove notifications by group
Alerter::remove("updates")?;

// List notifications by group
let info = Alerter::list("updates")?;

// List all
let all = Alerter::list("ALL")?;
```

## API Reference

### `Alerter`

| Method | Description |
|---|---|
| `new(message)` | Create a new notification with a message (required) |
| `title(&str)` | Notification title (default: "Terminal") |
| `subtitle(&str)` | Notification subtitle |
| `sound(&str)` | Sound name (`"default"` for system sound) |
| `actions(Vec<&str>)` | Action buttons (comma-separated in dropdown if multiple) |
| `dropdown_label(&str)` | Label for the actions dropdown |
| `reply(&str)` | Show reply field with placeholder text |
| `close_label(&str)` | Custom close button label |
| `group(&str)` | Group ID for notification replacement |
| `sender(&str)` | Bundle ID to impersonate (default: `com.apple.Terminal`) |
| `app_icon(&str)` | URL or path for app icon image |
| `content_image(&str)` | URL or path for attached image |
| `timeout(u32)` | Auto-close after N seconds |
| `json(bool)` | Output as JSON |
| `delay(u32)` | Deliver after N seconds |
| `at(&str)` | Deliver at time (`"HH:mm"` or `"yyyy-MM-dd HH:mm"`) |
| `ignore_dnd(bool)` | Send even if Do Not Disturb is on |
| `binary_path(path)` | Override the bundled binary path (for testing) |
| `send()` | Send the notification, returns `Result<AlerterResponse, AlerterError>` |
| `remove(group_id)` | Remove notifications by group ID |
| `list(group_id)` | List notifications by group ID (or `"ALL"`) |

### `AlerterResponse`

| Field | Type | Description |
|---|---|---|
| `activation_type` | `String` | What happened: `@CONTENTCLICKED`, `@CLOSED`, `@TIMEOUT`, or an action label |
| `activation_value` | `Option<String>` | Reply text (when using reply notifications) |

### `AlerterError`

| Variant | Description |
|---|---|
| `BinaryExtraction(String)` | Failed to extract the embedded binary |
| `ProcessSpawn(String)` | Failed to spawn the alerter process |
| `Runtime(String)` | Alerter returned a non-zero exit code |

## How it works

1. **Build time**: `build.rs` runs `swift build -c release` on the vendored `alerter/` submodule
2. **Embedding**: The compiled binary is included via `include_bytes!`
3. **Runtime**: On first use, the binary is extracted to `~/Library/Caches/alerter_rs/` with a SHA256-based filename for cache invalidation
4. **Invocation**: Each API call spawns the cached binary as a subprocess with the appropriate CLI flags

## Versioning

The crate version mirrors the bundled alerter version (e.g., alerter v26.5 = crate `26.5.x`).

## License

This crate wraps [alerter](https://github.com/vjeantet/alerter) by Julien Blanchard (vjeantet), which is licensed under the MIT License.
