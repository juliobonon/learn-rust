use std::process::Command;
use std::io::{self, Write};

// remove all docker images and volumes
fn dockerclear(){
    let docker_output = Command::new("docker")
                                .arg("system")
                                .arg("prune")
                                .arg("--all")
                                .arg("--volumes")
                                .arg("--force")
                                .output()
                                .expect("failed to docker clean");

    io::stdout().write_all(&docker_output.stdout).unwrap();
    io::stderr().write_all(&docker_output.stderr).unwrap();
}

// list how many mbs will be cleared, then apt-get clean
fn aptgetclean(){
    let output = Command::new("sudo")55
                         .arg("/usr/bin/du")
                         .arg("-sh")
                         .arg("/var/cache/apt")
                         .output()
                         .expect("failed to execute process");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    let apt_output = Command::new("sudo")
                         .arg("apt-get")
                         .arg("clean")
                         .output()
                         .expect("failed to execute process");

    io::stdout().write_all(&apt_output.stdout).unwrap();
    io::stderr().write_all(&apt_output.stderr).unwrap();
}

fn main() {
    aptgetclean();
    dockerclear();
}
