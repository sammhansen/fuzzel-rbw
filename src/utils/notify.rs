use std::io::Error;
use std::process::{exit, Command};

use anyhow::Result;

use crate::config::parser::parse_config_file;
use crate::config::default::UserConfig;

// takes an icon path, summary and body and sends a notification through notify-send
pub fn send(icon: &str, summary: &str, body: String) -> Result<(), Error> {
    let user_config: UserConfig = match parse_config_file() {
        Ok(user_config) => user_config,
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        },
    };

    // dont send notifications when its disabled in the config file
    if !user_config.notifications {
        return Ok(());
    }

    let mut args: Vec<String> = Vec::new();

    args.push(String::from("-i"));
    args.push(String::from(icon));
    args.push(summary.to_string());
    args.push(body);

    let send_result = Command::new("notify-send").args(args).output();

    match send_result {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
