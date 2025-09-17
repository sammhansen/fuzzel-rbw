use crate::fuzzel;
use crate::rbw;

use std::collections::HashMap;
use std::io::Error;

// uses the name as key to get the value Vec<String> of users
// joins the users into a String then pipes it to fuzzel
// passes both the name and user to get::password
pub fn get_user(name_choice: String, name_to_users: HashMap<String, Vec<String>>) -> Result<(), Error> {
    if let Some(users) = name_to_users.get(&name_choice) {
        let mut users = users.clone();

        // sorts users from a-z
        users.sort();

        let users = users.join("\n");

        let user: String = fuzzel::fuzzel::show(users.clone())?;

        rbw::get::password(name_choice, user)
    } else {
        Err(Error::new(
            std::io::ErrorKind::NotFound,
            "No users found for the given name",
        ))
    }
}
