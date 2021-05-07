use std::process::Command;
use std::io::{self, Write};

fn main() {
    let output = Command::new("sudo")
                         .arg("/usr/bin/du")
                         .arg("-sh")
                         .arg("/var/cache/apt")
                         .output()
                         .expect("failed to execute process");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    let apt_output = Command::new("sudo")
                         .arg("apt-get")
                         .arg("clean")
                         .output()
                         .expect("failed to execute process");

    println!("status: {}", apt_output.status);
    io::stdout().write_all(&apt_output.stdout).unwrap();
    io::stderr().write_all(&apt_output.stderr).unwrap();

}
