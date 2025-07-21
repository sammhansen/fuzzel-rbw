use crate::fuzzel;
use std::process::{Command, Stdio};

pub fn main() {
    let json = fuzzel::fetch();

    let username = json.username;
    let password = json.password;
    let _totp = json.totp;

    autotype(username, password);
}

pub fn autotype(username: String, password: String) {
    let sequence = format!("{username}\t{password}");

    let mut child = Command::new("wtype")
        .arg(&sequence)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute wtype");

    let status = child.wait().expect("Failed to wait on wtype");

    if !status.success() {
        eprintln!("{status}");
    }
}
