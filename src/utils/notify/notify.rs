use std::{
    io::ErrorKind,
    path::PathBuf,
    process::{self, Command},
};

pub fn send(icon: PathBuf, summary: &str, body: String) {
    let mut args: Vec<String> = Vec::new();

    args.push(String::from("-i"));
    args.push(String::from(icon.to_string_lossy()));
    args.push(summary.to_string());
    args.push(body);

    let send_result = Command::new("notify-send").args(args).output();

    match send_result {
        Ok(_) => (),
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                eprintln!("Please install notify-send");
            } else {
                process::exit(1);
            }
        }
    }
}
