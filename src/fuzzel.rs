use crate::rbw;
use crate::structs::entry::Entry;
use crate::utils::json::to_json;
use std::process::{Command, Stdio};

pub fn spawn() -> String {
    let (mut rbw_child, rbw_stdout) = rbw::spawn();

    let child = Command::new("fuzzel")
        .args(["--dmenu", "--placeholder", "> helloow"])
        .stdin(Stdio::from(rbw_stdout))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run fuzzel");

    let _ = rbw_child.wait().expect("Failed to wait on rbw");

    let child_out = child.wait_with_output().expect("Failed to wait on fuzzel");

    let choice_raw = String::from_utf8_lossy(&child_out.stdout);
    let choice = choice_raw.trim();

    choice.to_string()
}

pub fn fetch() -> Entry {
    let choice = spawn();

    let fetch_json = Command::new("rbw")
        .args(["get", "--raw", choice.as_str()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run rbw");

    let unprocessed = fetch_json.wait_with_output().expect("command rbw failed");
    let data = String::from_utf8_lossy(&unprocessed.stdout);

    let data_string = data.to_string();

    to_json(data_string)
}
