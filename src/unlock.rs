// use crate::fuzzel;
use crate::insert;
use std::process::Command;

pub fn unlock() {
    let unlock = Command::new("rbw")
        .arg("unlock")
        .output()
        .expect("Failed to unlock rbw");

    if !unlock.status.success() {
        println!("{}", String::from_utf8_lossy(&unlock.stderr));
    } else {
        // fuzzel::fetch();
        insert::main();
    };
}
