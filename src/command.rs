use std::{
    io::{Error, Write},
    process::{Command, Stdio},
};

// takes a command and arguments and returns the stdout as a String
pub fn no_std_in(command: &str, args: Vec<String>) -> Result<String, Error> {
    let command_output = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .output()?;

    let command_stdout: String = String::from_utf8_lossy(&command_output.stdout)
        .trim()
        .to_string();

    Ok(command_stdout)
}

// takes a command , arguments and stdin and returns the stdout as a String
pub fn with_std_in(command: &str, args: Vec<String>, stdin: String) -> Result<String, Error> {
    let mut command_child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // borrows stdin as mutable
    let command_child_stdin = command_child
        .stdin
        .as_mut()
        .expect("failed to borrow stdin as mutable");

    // writes to stdin as bytes
    command_child_stdin.write_all(stdin.as_bytes())?;

    let command_output = command_child.wait_with_output()?;

    let command_stdout: String = String::from_utf8_lossy(&command_output.stdout)
        .trim()
        .to_string();

    Ok(command_stdout)
}
