use std::process::Command;
// pub fn sys_cmd<F>(f: F) where F: FnOnce() {
//     f();
// }

pub fn sys_cmd(exec: &str) -> std::io::Result<()> {
    Command::new(exec).status().map(|_| ())
}