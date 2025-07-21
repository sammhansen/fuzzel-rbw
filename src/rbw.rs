use std::process::{Child, ChildStdout, Command, Stdio};

pub fn spawn() -> (Child, ChildStdout) {
    let mut rbw_child = Command::new("rbw")
        .arg("list")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn rbw");

    let rbw_stdout = rbw_child.stdout.take().expect("Failed to read rbw_output");

    (rbw_child, rbw_stdout)
}
