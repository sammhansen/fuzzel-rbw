use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process;

use crate::config;
use crate::config::default;
use crate::config::default::UserConfig;
use crate::utils::json;

// checks if the config exists at CONFIG_DIR, creates it and writes the default config to it
fn check_config_file_existence(path: PathBuf) -> Result<File, Error> {
    let file = File::open(&path);
    let config_dir: PathBuf = PathBuf::from(path);

    match file {
        Ok(file) => Ok(file),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => default::create_config_file(config_dir.clone()),
            _ => {
                eprintln!("{err}");
                Err(err)
            }
        },
    }
}

// parse the config file , return UserConfig if Ok
pub fn parse_config_file() -> Result<UserConfig, Error> {
    let config_dir: PathBuf = config::path::expand_path(config::constants::CONFIG_DIR);

    let config_file = check_config_file_existence(config_dir);

    match config_file {
        Ok(mut file) => match json::from_json::read_json_from_config_file(&mut file) {
            Ok(user_config) => Ok(user_config),
            Err(err) => {
                eprintln!("parse_config_file(): {err}");
                process::exit(1);
            }
        },
        Err(err) => match err.kind() {
            _ => Err(err),
        },
    }
}
