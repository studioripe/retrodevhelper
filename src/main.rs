use std::{
    env::{self, current_dir},
    io,
};

use inquire::{error::InquireResult, Select, Text};
use rust_i18n::t;
use serde_derive::{Deserialize, Serialize};

mod genesis;

rust_i18n::i18n!("locales");

#[derive(Deserialize, Serialize)]
struct Project {
    name: String,
    sdk: String,
}

fn main() -> InquireResult<()> {
    let args: Vec<String> = env::args().collect();
    print_logo();
    if &args.len() > &1 {
        let query: &str = &args[1];
        match query {
            "init" => init_project()?,
            "build" => build_project()?,
            "run" => run_project()?,
            _ => help()?,
        }
    } else {
        help()?;
    }

    Ok(())
}

fn get_consoles() -> Vec<&'static str> {
    vec!["Genesis/Mega Drive"]
}

fn build_project() -> io::Result<()> {
    let project: Project = get_project()?;
    let sdk = project.sdk.as_str();
    match sdk {
        "SGDK" => genesis::build::project().expect("Error Building Project"),
        _ => println!("SDK not found"),
    }

    Ok(())
}

fn run_project() -> io::Result<()> {
    let project: Project = get_project()?;
    let sdk = project.sdk.as_str();
    match sdk {
        "SGDK" => genesis::run::project().expect("Error Running Project"),
        _ => println!("SDK not found"),
    }

    Ok(())
}

fn help() -> io::Result<()> {
    println!("{}", t!("help_title").as_str());
    println!("{}", t!("help_subtitle").as_str());
    println!("{}", t!("init_help").as_str());
    println!("{}", t!("build_help").as_str());
    println!("{}", t!("run_help").as_str());
    Ok(())
}

fn init_project() -> io::Result<()> {
    let _name: String = Text::new(t!("project_name").as_str())
        .with_help_message(t!("project_help").as_str())
        .prompt()
        .expect("Error entering name");

    let _console = Select::new(t!("console_target").as_str(), get_consoles())
        .prompt()
        .expect("Error selecting console");

    genesis::create::project(_name.as_str());
    Ok(())
}

fn get_project() -> io::Result<Project> {
    let project: Project = {
        let dir = current_dir().unwrap();
        let dir_value = dir.display();

        let project = std::fs::read_to_string(format!("{dir_value}/project.json"))?;

        serde_json::from_str::<Project>(&project).unwrap()
    };
    Ok(project)
}

fn print_logo() {
    println!();
    println!("█▀█ █▀▀ ▀█▀ █▀█ █▀█ ▄▄ █▀▄ █▀▀ █░█ ▄▄ █░█ █▀▀ █░░ █▀█ █▀▀ █▀█");
    println!("█▀▄ ██▄ ░█░ █▀▄ █▄█ ░░ █▄▀ ██▄ ▀▄▀ ░░ █▀█ ██▄ █▄▄ █▀▀ ██▄ █▀▄");
    println!();
}
