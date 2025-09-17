use std::{io::Error, process::Command};

// uses wtype to type in the user and pass
pub fn key_in(user: String, pass: String) -> Result<(), Error> {
    let args = format!("{}\t{}", user.trim(), pass.trim());
    Command::new("wtype").arg(args).output()?;
    Ok(())
}
