use dialoguer::{theme::ColorfulTheme, Confirm};
use std::process::Command;
use std::str::from_utf8;

fn main() {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to hibernate?")
        .interact()
        .unwrap()
    {
        let res = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "runas /usr:administrator /savecred \"shutdown /h\""])
                .output()
                .expect("Failed to elevate permission and execute the process")
        } else {
            Command::new("sh")
                .args(["-c", "sudo systemctl hibernate"])
                .output()
                .expect("Failed to execute the process")
        };
        if !res.status.success() {
            println!("status: {}", res.status.to_string());
        };
        if !res.stdout.is_empty() {
            println!("stdout: {}", match_u8(&res.stdout[..]));
        };
        if !res.stderr.is_empty() {
            println!("stderr: {}", match_u8(&res.stderr[..]));
        };
    }
}

fn match_u8(output: &[u8]) -> String {
    match from_utf8(output) {
        Ok(v) => format!("\n{}", v),
        Err(_) => "invalid UTF-8 sequence".to_string(),
    }
}
