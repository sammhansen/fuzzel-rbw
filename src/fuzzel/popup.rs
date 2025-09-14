use std::{
    io::{Error, Write},
    process::{self, Command, Stdio},
};

use crate::config::default::UserConfig;
use crate::config::parser::parse_config_file;

// runs fuzzel and pipes rbw list to it
pub fn show_fuzzel(command_rbw_list_stdout: String) -> Result<String, Error> {
    // gets the config file
    let user_config: UserConfig = match parse_config_file() {
        Ok(user_config) => user_config,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    // store all arguments into a vector and pass it to the command
    let mut fuzzel_args: Vec<String> = Vec::new();

    fuzzel_args.push(String::from("--dmenu"));
    fuzzel_args.push(String::from("--prompt"));
    fuzzel_args.push(String::from(&user_config.prompt));
    fuzzel_args.push(String::from("--placeholder"));
    fuzzel_args.push(String::from(&user_config.placeholder));
    fuzzel_args.push(String::from("--lines"));
    fuzzel_args.push(user_config.lines.to_string());

    // spawn fuzzel
    let mut command_fuzzel = Command::new("fuzzel")
        .args(fuzzel_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("An error occured");

    let command_fuzzel_stdin = command_fuzzel.stdin.as_mut().unwrap();

    command_fuzzel_stdin.write_all(command_rbw_list_stdout.as_bytes())?;

    let command_fuzzel_output = command_fuzzel.wait_with_output()?;

    // trims the whitespace at the end and converts it into a String
    let command_fuzzel_stdout: String = String::from_utf8_lossy(&command_fuzzel_output.stdout)
        .trim()
        .to_string();

    Ok(command_fuzzel_stdout)
}
