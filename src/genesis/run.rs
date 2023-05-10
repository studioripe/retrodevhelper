use std::{env::current_dir, process::Command};

use spinoff::{spinners, Color, Spinner};

pub fn project(project_name: &str) {
    println!("Running Project {project_name}");

    let dir = current_dir().unwrap();
    let dir_value = dir.display();

    let a = format!("{dir_value}/out/rom.bin");
    let spinner = Spinner::new(spinners::Dots, "Running Project...", Color::Blue);

    let output = Command::new("open")
        .args(["-a", "Genesis Plus", &a])
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        spinner.success("Project Started");
    } else {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        spinner.fail("Failed to Run Project");
    }
}
