use std::env;
use std::process::{Command, Stdio};
use std::io::Write;

fn main() {
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");

    let output = format!(
        "{}\n{}{}{}\n{}",
        "    /*//////////////////////////////////////////////////////////////",
        "    ",
        (0..(64 - input.len()) / 2).map(|_| " ").collect::<String>(),
        input.to_uppercase(),
        "    //////////////////////////////////////////////////////////////*/"
    );

    println!("{}", &output); // Print the header to console.

    // Detect if running under WSL
    let is_wsl = env::var("WSL_DISTRO_NAME").is_ok();

    if is_wsl {
        // Use Windows clip.exe to set the clipboard
        if let Err(e) = set_clipboard_windows(&output) {
            eprintln!("Failed to set clipboard using clip.exe: {}", e);
        }
    } else {
        // Attempt to use the clipboard crate (likely won't work in WSL)
        if let Err(e) = set_clipboard_unix(&output) {
            eprintln!("Failed to set clipboard using clipboard crate: {}", e);
        }
    }
}

fn set_clipboard_windows(output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut clip = Command::new("clip.exe")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn clip.exe");

    clip.stdin
        .as_mut()
        .expect("Failed to open stdin")
        .write_all(output.as_bytes())?;

    clip.wait()?;
    Ok(())
}

fn set_clipboard_unix(output: &str) -> Result<(), Box<dyn std::error::Error>> {
    use clipboard::{ClipboardContext, ClipboardProvider};

    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(output.to_owned())?;
    Ok(())
}
