use std::io::Error;
use std::process::Command;

// takes an icon path, summary and body and sends a notification through notify-send
pub fn send(icon: &str, summary: &str, body: String) -> Result<(), Error> {
    let mut args: Vec<String> = Vec::new();

    args.push(String::from("-i"));
    args.push(String::from(icon));
    args.push(summary.to_string());
    args.push(body);

    let send_result = Command::new("notify-send").args(args).output();

    match send_result {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
