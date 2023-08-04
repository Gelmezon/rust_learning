use std::process::Command;

fn main() {
    let mut output = Command::new("ls");

    output.status().expect("failed to execute process");

    output.current_dir("/data");

    output.status().expect("failed to execute process");
}
