use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::{File, create_dir_all},
    io::{Error, Write},
    path::PathBuf,
    process,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserConfig {
    pub placeholder: String,
    pub prompt: String,
    pub lines: u8,
    pub notifications: bool,
}

// creates the config file if it does not exist and writes the default config file to it
pub fn create_config_file(path: PathBuf) -> Result<File, Error> {
    // creates the parent directory before trying to write the file
    if let Some(parent_dir) = path.parent() {
        create_dir_all(parent_dir)?;
    }

    let mut file = match File::create_new(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    let serialize_user_config: UserConfig = UserConfig {
        placeholder: "select an entry".to_owned(),
        prompt: "> ".to_owned(),
        lines: 6,
        notifications: true,
    };

    // write to the config file with indentation
    serde_json::to_writer_pretty(&mut file, &serialize_user_config)?;
    file.flush()?;

    Ok(file)
}
