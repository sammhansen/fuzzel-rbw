use crate::rbw;
use crate::structs::entry::Entry;
use crate::utils::json::to_json;
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

pub fn spawn() -> (String, String) {
    let list = rbw::spawn();
    let list_clone = list.clone();

    let mut child = Command::new("fuzzel")
        .args(["--dmenu", "--placeholder", "> helloow"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run fuzzel");

    let mut stdin = child.stdin.take().expect("Failed to take `fuzzel` stdin");

    // write `rbw list` stdout to `fuzzel` stdin
    std::thread::spawn(move || {
        stdin
            .write_all(list_clone.as_bytes())
            .expect("Failed to write to fuzzels stdin");
    });

    let child_out = child.wait_with_output().expect("Failed to wait on fuzzel");

    let choice_raw = String::from_utf8_lossy(&child_out.stdout);
    let choice = choice_raw.trim();

    (choice.to_string(), list)
}

// handle cases where an entry name appears twice
// ie: www.instagram.com for user1@gmail.com and www.instagram.com for user2@gmail.com
//
pub fn handle_dups(choice: String, list: String) -> (bool, String, String) {
    let is_dup: bool = false;
    let count = list.match_indices(&choice).count();
    let mut username = String::new();

    if count > 1 {
        let is_dup: bool = true;

        let mut search_child = Command::new("rbw")
            .args(["search", choice.as_str()])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn `rbw search`");

        let mut search_stdout = search_child
            .stdout
            .take()
            .expect("Failed to take `rbw search` stdout");

        let mut possibilities = String::new();

        search_stdout
            .read_to_string(&mut possibilities)
            .expect("Failed to read stdout to string");

        let _ = search_child.wait().expect("Failed to wait on `rbw search`");

        // replace the @uri with nothing so we only have the username field
        let entry_name = format!("@{choice}");
        let possible_choices = possibilities.replace(&entry_name, "");

        // pipe usernames of duplicate entries to fuzzel
        let mut possible_child = Command::new("fuzzel")
            .args([
                "--dmenu",
                "--placeholder",
                "choose the specific entry",
                "--prompt",
                "> ",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to pipe possible entries to fuzzel");

        let mut possible_stdin = possible_child
            .stdin
            .take()
            .expect("Failed to take possible_child stdin");

        // write the possible usernames to fuzzels stdin
        std::thread::spawn(move || {
            possible_stdin
                .write_all(possible_choices.as_bytes())
                .expect("Failed to write to possible_child stdin");
        });

        let mut possible_stdout = possible_child
            .stdout
            .take()
            .expect("Failed to take possible_child stdout");

        // store specified username in username
        let possible_result = possible_stdout.read_to_string(&mut username);

        if let Err(err) = possible_result {
            eprintln!("{err}");
        }

        let _wait_possible = possible_child
            .wait()
            .expect("Failed to wait on possible_child");

        println!("The choice is {choice} and the username is {username}");

        return (is_dup, choice, username);
    }
    (is_dup, choice, username)
}

pub fn fetch() -> Entry {
    let (choice, list) = spawn();

    // let choice_clone = choice.clone();
    let (is_dup, choice, username) = handle_dups(choice, list);

    // let dup = if is_dup
    let mut args = vec!["get", "--raw"];

    // dynamic args vector based on is_dup
    if is_dup {
        args.push(choice.as_str().trim());
        args.push(username.as_str().trim());
    } else {
        args.push(choice.as_str().trim());
    }

    // fetch raw json
    let fetch_json = Command::new("rbw")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run rbw");

    let unprocessed = fetch_json.wait_with_output().expect("command rbw failed");
    let data = String::from_utf8_lossy(&unprocessed.stdout);

    let data_string = data.to_string();

    to_json(data_string)
}
