use std::process::Command;

pub fn run_ls() -> std::io::Result<()> {
    Command::new("ls").status().map(|_| ())
}