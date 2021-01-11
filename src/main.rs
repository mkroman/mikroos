// Build script inspired by the one in gamozolabs x86_64 kernel:
// https://github.com/gamozolabs/chocolate_milk/blob/69640cc31e4cd96cbd162ab92fe5cf701c454f74/src/main.rs

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

#[cfg(unix)]
use std::os::unix::process::CommandExt;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// Runs the given `cmdline` to determine whether the tool called `name` is installed and usable
///
/// Returns `Some(())` on success
fn run_quiet(cmd: &str, args: &[&str]) -> Option<()> {
    if Command::new(cmd)
        .args(args)
        .stdout(Stdio::null())
        .status()
        .ok()?
        .success()
    {
        return Some(());
    } else {
        return None;
    }
}

fn gdb() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("gdb").args(&["-x", "./gdbinit"]).exec();

    Ok(())
}

fn build() -> Result<(), Box<dyn std::error::Error>> {
    // Create the build folders if they don't exist
    fs::create_dir_all("build/bootloader")?;

    // Build the bootloader
    let bootloader_dir = Path::new("bootloader");

    println!("Building bootloader");

    if !Command::new("xargo")
        .current_dir(bootloader_dir)
        .arg("build")
        .arg("--target-dir")
        .arg("../build/bootloader")
        .arg("--release")
        .arg("-v")
        .status()?
        .success()
    {
        return Err("Could not build bootloader".into());
    }

    // Verify that bouffalo-cli is installed
    if !run_quiet("bouffalo-cli", &["--version"]).is_some() {
        return Err("bouffalo-cli tool is not installed".into());
    }

    // Convert the elf to a firmware image
    if !Command::new("bouffalo-cli")
        .arg("elf2image")
        .arg("build/bootloader/riscv32imac-unknown-none-elf/release/bootloader")
        .status()?
        .success()
    {
        return Err("Could not convert elf to firmware image".into());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        build()?;
    } else {
        let action = args.get(1).unwrap();

        match action.as_str() {
            "build" => build()?,
            "gdb" => gdb()?,
            _ => {
                eprintln!("Unknown action: {}", action);
            }
        }
    }

    Ok(())
}
