use std::io::{Error, ErrorKind};
use std::process::{Command, Output};

use crate::config::constants::FRBW_ICON_NAME;
use crate::utils::notify;

fn check(command: String) -> Result<Output, Error> {
    Command::new(command).arg("--version").output()
}

// checks if all dependencies are installed
// fuzzel, wtype, rbw, notify-send
pub fn dependencies() -> Result<(), Error> {
    let deps: Vec<String> = vec![
        String::from("wtype"),
        String::from("fuzzel"),
        String::from("rbw"),
        String::from("notify-send"),
    ];

    let mut missing_deps: Vec<String> = Vec::new();

    for dep in deps {
        if let Err(err) = check(dep.clone()) {
            match err.kind() {
                ErrorKind::NotFound => {
                    missing_deps.push(dep);
                }
                _ => {
                    let body: String = format!("Err: {err}");
                    eprintln!("{err}");

                    notify::send(FRBW_ICON_NAME, "Fuzzel RBW", body)?
                }
            }
        }
    }

    if !missing_deps.is_empty() {
        let body: String = format!("Missing dependencies: {}", missing_deps.join(", "));

        notify::send(FRBW_ICON_NAME, "Fuzzel RBW", body)?
    }

    Ok(())
}
