use alerter_rs::Alerter;

#[test]
fn new_creates_builder_with_message() {
    let alerter = Alerter::new("Hello world");
    let args = alerter.build_args();
    assert!(
        args.contains(&"--message".to_string()),
        "Args should contain --message flag"
    );
    assert!(
        args.contains(&"Hello world".to_string()),
        "Args should contain the message text"
    );
}

#[test]
fn title_sets_title_argument() {
    let args = Alerter::new("msg").title("My Title").build_args();
    assert!(args.contains(&"--title".to_string()));
    assert!(args.contains(&"My Title".to_string()));
}

#[test]
fn subtitle_sets_subtitle_argument() {
    let args = Alerter::new("msg").subtitle("Sub").build_args();
    assert!(args.contains(&"--subtitle".to_string()));
    assert!(args.contains(&"Sub".to_string()));
}

#[test]
fn sound_sets_sound_argument() {
    let args = Alerter::new("msg").sound("default").build_args();
    assert!(args.contains(&"--sound".to_string()));
    assert!(args.contains(&"default".to_string()));
}

#[test]
fn actions_sets_actions_argument() {
    let args = Alerter::new("msg")
        .actions(vec!["Yes", "No", "Maybe"])
        .build_args();
    assert!(args.contains(&"--actions".to_string()));
    // Actions should be comma-separated
    assert!(
        args.iter()
            .any(|a: &String| a.contains("Yes") && a.contains("No")),
        "Actions should be formatted as comma-separated values"
    );
}

#[test]
fn dropdown_label_sets_dropdown_argument() {
    let args = Alerter::new("msg")
        .dropdown_label("Choose one")
        .build_args();
    assert!(args.contains(&"--dropdown-label".to_string()));
    assert!(args.contains(&"Choose one".to_string()));
}

#[test]
fn reply_sets_reply_argument() {
    let args = Alerter::new("msg").reply("Type here...").build_args();
    assert!(args.contains(&"--reply".to_string()));
    assert!(args.contains(&"Type here...".to_string()));
}

#[test]
fn close_label_sets_close_label_argument() {
    let args = Alerter::new("msg").close_label("Dismiss").build_args();
    assert!(args.contains(&"--close-label".to_string()));
    assert!(args.contains(&"Dismiss".to_string()));
}

#[test]
fn group_sets_group_argument() {
    let args = Alerter::new("msg").group("updates").build_args();
    assert!(args.contains(&"--group".to_string()));
    assert!(args.contains(&"updates".to_string()));
}

#[test]
fn sender_sets_sender_argument() {
    let args = Alerter::new("msg").sender("com.apple.Safari").build_args();
    assert!(args.contains(&"--sender".to_string()));
    assert!(args.contains(&"com.apple.Safari".to_string()));
}

#[test]
fn app_icon_sets_app_icon_argument() {
    let args = Alerter::new("msg")
        .app_icon("/path/to/icon.png")
        .build_args();
    assert!(args.contains(&"--app-icon".to_string()));
    assert!(args.contains(&"/path/to/icon.png".to_string()));
}

#[test]
fn content_image_sets_content_image_argument() {
    let args = Alerter::new("msg")
        .content_image("/path/to/photo.png")
        .build_args();
    assert!(args.contains(&"--content-image".to_string()));
    assert!(args.contains(&"/path/to/photo.png".to_string()));
}

#[test]
fn timeout_sets_timeout_argument() {
    let args = Alerter::new("msg").timeout(10).build_args();
    assert!(args.contains(&"--timeout".to_string()));
    assert!(args.contains(&"10".to_string()));
}

#[test]
fn json_sets_json_argument() {
    let args = Alerter::new("msg").json(true).build_args();
    assert!(args.contains(&"--json".to_string()));
}

#[test]
fn json_false_does_not_set_json_argument() {
    let args = Alerter::new("msg").json(false).build_args();
    assert!(
        !args.contains(&"--json".to_string()),
        "json(false) should not include --json flag"
    );
}

#[test]
fn delay_sets_delay_argument() {
    let args = Alerter::new("msg").delay(30).build_args();
    assert!(args.contains(&"--delay".to_string()));
    assert!(args.contains(&"30".to_string()));
}

#[test]
fn at_sets_at_argument() {
    let args = Alerter::new("msg").at("14:30").build_args();
    assert!(args.contains(&"--at".to_string()));
    assert!(args.contains(&"14:30".to_string()));
}

#[test]
fn ignore_dnd_sets_ignore_dnd_argument() {
    let args = Alerter::new("msg").ignore_dnd(true).build_args();
    assert!(args.contains(&"--ignore-dnd".to_string()));
}

#[test]
fn ignore_dnd_false_does_not_set_argument() {
    let args = Alerter::new("msg").ignore_dnd(false).build_args();
    assert!(
        !args.contains(&"--ignore-dnd".to_string()),
        "ignore_dnd(false) should not include --ignore-dnd flag"
    );
}

#[test]
fn builder_methods_chain_fluently() {
    let args = Alerter::new("Hello")
        .title("Title")
        .subtitle("Subtitle")
        .sound("default")
        .actions(vec!["OK", "Cancel"])
        .dropdown_label("Options")
        .close_label("Close")
        .group("test-group")
        .sender("com.test.app")
        .app_icon("/icon.png")
        .content_image("/image.png")
        .timeout(30)
        .json(true)
        .delay(5)
        .ignore_dnd(true)
        .build_args();

    assert!(args.contains(&"--message".to_string()));
    assert!(args.contains(&"--title".to_string()));
    assert!(args.contains(&"--subtitle".to_string()));
    assert!(args.contains(&"--sound".to_string()));
    assert!(args.contains(&"--actions".to_string()));
    assert!(args.contains(&"--dropdown-label".to_string()));
    assert!(args.contains(&"--close-label".to_string()));
    assert!(args.contains(&"--group".to_string()));
    assert!(args.contains(&"--sender".to_string()));
    assert!(args.contains(&"--app-icon".to_string()));
    assert!(args.contains(&"--content-image".to_string()));
    assert!(args.contains(&"--timeout".to_string()));
    assert!(args.contains(&"--json".to_string()));
    assert!(args.contains(&"--delay".to_string()));
    assert!(args.contains(&"--ignore-dnd".to_string()));
}

#[test]
fn minimal_builder_only_has_message() {
    let args = Alerter::new("Just a message").build_args();
    assert_eq!(
        args.len(),
        2,
        "Minimal builder should only produce --message and the message text"
    );
    assert_eq!(args[0], "--message");
    assert_eq!(args[1], "Just a message");
}
