use crate::insert;
use std::process::Command;

pub fn unlock() {
    let unlock = Command::new("rbw")
        .arg("unlock")
        .output()
        .expect("Failed to unlock rbw");

    // send a notification when unable to unlock the rbw database

    if !unlock.status.success() {
        let icon_path: String = format!("/usr/share/fuzzel-rbw/assets/logos/bitwarden.svg");

        let unlock_error = String::from_utf8_lossy(&unlock.stderr).to_owned();

        Command::new("notify-send")
            .args(["-i", &icon_path, "Fuzzel RBW", &unlock_error])
            .spawn()
            .expect("Failed to execute notify send");
    } else {
        insert::main();
    };
}
