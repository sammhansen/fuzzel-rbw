use crate::constants;
use crate::utils::notify;
use std::{path::PathBuf, process::{self, exit, Command}};

pub fn unlock() {
    let rbw_unlock = Command::new("rbw").arg("unlock").spawn();

    match rbw_unlock {
        Ok(_) => {
            println!("unlocking sucsexxful");
        }
        Err(error) => {
            let icon_path: PathBuf = PathBuf[]
            notify::notify::send(constants::FRBW_ICON.as, summary, body);
            process::exit(1);
        }
    }
}
