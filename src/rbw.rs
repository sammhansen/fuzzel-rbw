use std::{
    io::Read,
    process::{ChildStdout, Command, Stdio},
};

fn process_list(mut stdout: ChildStdout) -> String {
    let mut list_string = String::new();

    // read `rbw list` stdout into a string
    stdout
        .read_to_string(&mut list_string)
        .expect("Failed to read string");

    list_string
}

pub fn spawn() -> String {
    // spawn `rbw list`
    let mut child = Command::new("rbw")
        .arg("list")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn rbw");

    let stdout = child.stdout.take().expect("Failed to read rbw_output");

    // pass `rbw list` stdout to process_list to get the list in a Vec<String>
    let list = process_list(stdout);

    let _ = child.wait().expect("Failed to wait on rbw");

    list
}
