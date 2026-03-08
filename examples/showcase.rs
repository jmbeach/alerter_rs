use alerter_rs::Alerter;
use alerter_rs::AlerterResponse;

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_default();

    match arg.as_str() {
        "basic" => {
            let result = Alerter::new("This is a basic notification")
                .title("Basic")
                .timeout(5)
                .send();
            print_result(result);
        }
        "sound" => {
            let result = Alerter::new("This notification has sound")
                .title("Sound")
                .sound("default")
                .timeout(5)
                .send();
            print_result(result);
        }
        "actions" => {
            let result = Alerter::new("Choose an option")
                .title("Actions")
                .actions(vec!["Yes", "No"])
                .close_label("Maybe")
                .send();
            print_result(result);
        }
        "dropdown" => {
            let result = Alerter::new("Select from the dropdown")
                .title("Dropdown")
                .dropdown_label("Pick one")
                .actions(vec!["Option A", "Option B", "Option C"])
                .timeout(5)
                .send();
            print_result(result);
        }
        "reply" => {
            let result = Alerter::new("Send a reply")
                .title("Reply")
                .reply("Type here...")
                .timeout(5)
                .send();
            print_result(result);
        }
        "icon" => {
            let result = Alerter::new("This notification has a custom icon")
                .title("Icon")
                .app_icon("/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/AlertNoteIcon.icns")
                .timeout(5)
                .send();
            print_result(result);
        }
        "content-image" => {
            let result = Alerter::new("This notification has a content image")
                .title("Content Image")
                .content_image("/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/AlertNoteIcon.icns")
                .timeout(5)
                .send();
            print_result(result);
        }
        "subtitle" => {
            let result = Alerter::new("This notification has a subtitle")
                .title("Subtitle")
                .subtitle("This is a subtitle")
                .timeout(5)
                .send();
            print_result(result);
        }
        "group" => {
            let result = Alerter::new("This notification belongs to a group")
                .title("Group")
                .group("showcase-group")
                .timeout(5)
                .send();
            print_result(result);
        }
        "sender" => {
            let result = Alerter::new("This notification has a custom sender")
                .title("Sender")
                .sender("com.apple.Safari")
                .timeout(5)
                .send();
            print_result(result);
        }
        "json" => {
            let result = Alerter::new("This notification returns JSON")
                .title("JSON")
                .json(true)
                .timeout(5)
                .send();
            print_result(result);
        }
        "close-label" => {
            let result = Alerter::new("This notification has a custom close label")
                .title("Close Label")
                .close_label("Dismiss")
                .timeout(5)
                .send();
            print_result(result);
        }
        "ignore-dnd" => {
            let result = Alerter::new("This notification ignores Do Not Disturb")
                .title("Ignore DND")
                .ignore_dnd(true)
                .timeout(5)
                .send();
            print_result(result);
        }
        "remove" => match Alerter::remove("showcase-group") {
            Ok(()) => println!("Removed notifications for group 'showcase-group'"),
            Err(e) => eprintln!("Remove error: {e}"),
        },
        _ => {
            eprintln!("Usage: showcase <type>");
            eprintln!();
            eprintln!("Available notification types:");
            eprintln!("  basic          Basic notification with title and message");
            eprintln!("  sound          Notification with sound");
            eprintln!("  actions        Notification with action buttons");
            eprintln!("  dropdown       Notification with dropdown menu");
            eprintln!("  reply          Notification with reply field");
            eprintln!("  icon           Notification with custom app icon");
            eprintln!("  content-image  Notification with content image");
            eprintln!("  subtitle       Notification with subtitle");
            eprintln!("  group          Notification with group identifier");
            eprintln!("  sender         Notification with custom sender");
            eprintln!("  json           Notification with JSON output");
            eprintln!("  close-label    Notification with custom close label");
            eprintln!("  ignore-dnd     Notification that ignores Do Not Disturb");
            eprintln!("  remove         Remove notifications for a group");
            std::process::exit(1);
        }
    }
}

fn print_result(result: Result<AlerterResponse, alerter_rs::AlerterError>) {
    match result {
        Ok(response) => println!("{:?}", response),
        Err(e) => eprintln!("Notification error: {e}"),
    }
}
