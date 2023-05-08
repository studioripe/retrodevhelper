use std::{env::current_dir, process::Command};

use spinoff::{spinners, Color, Spinner};

pub fn project(project_name: &str) {
    println!("Building Project {project_name}");

    let dir = current_dir().unwrap();
    let dir_value = dir.display();

    let a = format!("{dir_value}:/m68k");
    let spinner = Spinner::new(spinners::Dots, "Building...", Color::Blue);

    let output = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &a,
            "-t",
            "registry.gitlab.com/doragasu/docker-sgdk:v1.80",
        ])
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        spinner.success("Build Successful");
    } else {
        println!("stderr: {}", String::from_utf8_lossy(&output.stdout));
        spinner.fail("Build Failed");
    }
}
