use std::process::{Command, Stdio};

pub(crate) fn current_branch() -> String {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    String::from_utf8(output.stdout).unwrap()
        .replace('\n', "")
}
