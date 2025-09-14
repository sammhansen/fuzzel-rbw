use std::process::Command;

// runs rbw list - returns the stdout as list or kills the program if an error occurs
pub fn rbw_list() -> String {
    let command_rbw_list = Command::new("rbw").arg("list").output();

    match command_rbw_list {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
