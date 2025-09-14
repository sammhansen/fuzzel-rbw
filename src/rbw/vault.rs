use crate::config;
use crate::fuzzel;
use crate::rbw;
use crate::utils::notify;

use std::{
    path::PathBuf,
    process::{self, Command, Stdio},
};

pub fn rbw_unlock() {
    // tries to unlock the database first - shows pinentry popup
    let command_rbw_unlock = Command::new("rbw")
        .arg("unlock")
        .stderr(Stdio::piped())
        .output()
        .expect("rbw unlock failed");

    // sends notification with stderr incase the command fails
    if !command_rbw_unlock.status.success() {
        let icon_path: PathBuf = PathBuf::from(config::constants::FRBW_ICON);
        let error: String = String::from_utf8_lossy(&command_rbw_unlock.stderr).to_string();

        notify::notify::send(icon_path, config::constants::FRBW_NAME, error);

        process::exit(1);
    } else {
        let command_rbw_list_stdout: String = rbw::fetch::rbw_list();

        let command_fuzzel = fuzzel::popup::show_fuzzel(command_rbw_list_stdout);

        if let Ok(fuzzel_choice) = command_fuzzel {
            println!("The choice is {fuzzel_choice}");
        }
    }
}
