use std::{env::current_dir, io, process::Command};

use rust_i18n::t;
use spinoff::{spinners, Color, Spinner};

pub fn project() -> io::Result<()> {
    let dir = current_dir().unwrap();
    let dir_value = dir.display();

    let a = format!("{dir_value}/out/rom.bin");
    let spinner = Spinner::new(spinners::Dots, t!("running_project"), Color::Blue);

    let output = Command::new("open")
        .args(["-a", "Genesis Plus", &a])
        .output()?;

    if output.status.success() {
        spinner.success(t!("project_started").as_str());
        Ok(())
    } else {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        spinner.fail(t!("run_fail").as_str());
        Ok(())
    }
}
