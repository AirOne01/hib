use clap;
use dialoguer::{theme::ColorfulTheme, Confirm};
use log::{info, warn};
use std::process::Command;
use std::str::from_utf8;

fn main() {
    simple_logger::init().unwrap();

    let mut verbose = false;

    match clap::Command::new("hibe")
        .about("System hibernation utility")
        .version("1.2.1")
        .author("AirOne01")
        .arg(
            clap::Arg::new("verbose")
                .long("verbose")
                .short('v')
                .alias("debug")
                .short_alias('d')
                .help("Show stdout, stderr and status")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
        .subcommand()
    {
        Some(("verbose", _)) => {
            verbose = true;
        }
        _ => {}
    };

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
                // .args(["-c", "sudo systemctl hibernate"])
                .args(["-c", "false"])
                .output()
                .expect("Failed to execute the process")
        };
        if verbose {
            info!("Status: {}", res.status);
        } else if !res.status.success() {
            warn!("Status: {}", res.status);
        };
        if !verbose || res.stdout.is_empty() {
            info!("STDOUT: {}", wipe_lb(match_u8(&res.stdout[..])));
        };
        if !verbose || res.stderr.is_empty() {
            info!("STDERR: {}", wipe_lb(match_u8(&res.stderr[..])));
        };
    }
}

fn match_u8(output: &[u8]) -> String {
    match from_utf8(output) {
        Ok(v) => format!("\n{}", v),
        Err(_) => "invalid UTF-8 sequence".to_string(),
    }
}

fn wipe_lb(output: String) -> String {
    if output.split("\n").all(|x| x == "") {
        // if the string is full of newlines
        String::new()
    } else {
        output
    }
}
