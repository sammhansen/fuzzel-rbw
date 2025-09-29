use std::{io::Error, process::Command};

// keys in a sequence of the username,tab,password then return
// this fixes some incognito tabs failing to work because \t and \n are actual simulated keypresses
pub fn key_in(user: String, pass: String) -> Result<(), Error> {
    Command::new("wtype").arg(user.trim()).status()?;
    Command::new("wtype").args(["-k", "Tab"]).status()?;
    Command::new("wtype").arg(pass.trim()).status()?;
    Command::new("wtype").args(["-k", "Return"]).status()?;
    Ok(())
}
