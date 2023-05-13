use std::{env::current_dir, process::Command};

use rust_i18n::t;
use spinoff::{spinners, Color, Spinner};

pub fn project() {
    let dir = current_dir().unwrap();
    let dir_value = dir.display();

    let build_dir = format!("{dir_value}:/m68k");
    let spinner = Spinner::new(spinners::Dots, t!("building"), Color::Blue);

    let output = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &build_dir,
            "-t",
            "registry.gitlab.com/doragasu/docker-sgdk:v1.80",
        ])
        .output()
        .expect(t!("build_fail").as_str());

    if output.status.success() {
        spinner.success(t!("build_sucess").as_str());
    } else {
        println!("stderr: {}", String::from_utf8_lossy(&output.stdout));
        spinner.fail(t!("build_fail").as_str());
    }
}
