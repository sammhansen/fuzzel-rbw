use std::io::Error;

use crate::command;
use crate::utils::wtype;

// gets the password
pub fn password(name: String, user: String) -> Result<(), Error> {
    let args: Vec<String> = vec![String::from("get"), name, user.clone()];

    // rbw get <name> <user>
    let pass = command::no_std_in("rbw", args)?;

    // pass the user and pass to wtype
    wtype::key_in(user, pass)
}
