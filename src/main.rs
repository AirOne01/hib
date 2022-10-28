use dialoguer::{theme::ColorfulTheme, Confirm};
use std::process::Command;

fn main() {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to hibernate?")
        .interact()
        .unwrap()
    {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "runas /usr:administrator /savecred \"shutdown /h\""])
                .output()
                .expect("Failed to elevate permission and execute the process");
        } else {
            Command::new("sh")
                .args(["-c", "sudo systemctl hibernate"])
                .output()
                .expect("Failed to execute the process");
        }
    }
}
