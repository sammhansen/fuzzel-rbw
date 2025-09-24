use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // initial handshake
    writeln!(stdout, "OK")?;
    stdout.flush()?;

    for line in stdin.lock().lines() {
        let line = line?;
        let mut parts = line.splitn(2, ' ');
        let cmd = parts.next().unwrap_or("").to_uppercase();
        let rest = parts.next().unwrap_or("");

        match cmd.as_str() {
            "GETPIN" => {
                // call fuzzel
                let output = Command::new("fuzzel")
                    .arg("--prompt-only")
                    .arg("󰌆  ")
                    .arg(rest)
                    .arg("--placeholder")
                    .arg("master password")
                    .arg("--password")
                    .arg("--dmenu")
                    .stdin(Stdio::null())
                    .output()?;

                let password = String::from_utf8_lossy(&output.stdout).trim().to_string();

                if password.is_empty() {
                    writeln!(stdout, "ERR 83886179 User canceled")?;
                } else {
                    writeln!(stdout, "D {}", password)?;
                    writeln!(stdout, "OK")?;
                }
                stdout.flush()?;
            }
            "BYE" => {
                writeln!(stdout, "OK")?;
                stdout.flush()?;
                break;
            }
            _ => {
                writeln!(stdout, "OK")?;
                stdout.flush()?;
            }
        }
    }

    Ok(())
}
