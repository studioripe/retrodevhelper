use std::env::{self, current_dir};

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
            "init" => init_project(),
            "build" => build_project(),
            "run" => run_project(),
            _ => help(),
        }
    } else {
        help();
    }

    Ok(())
}

fn get_consoles() -> Vec<&'static str> {
    vec!["Genesis / Mega Drive"]
}

fn build_project() {
    let project: Project = get_project();
    genesis::build::project(&project.name);
}

fn run_project() {
    let project: Project = get_project();
    genesis::run::project(&project.name);
}

fn help() {
    println!("Help");
    println!("Commands:");
    println!("init - Start a New Project");
    println!("build - Build a Project");
    println!("run - Run a Project");
}

fn init_project() {
    let _name: String = Text::new(t!("project_name").as_str())
        .with_help_message(t!("project_help").as_str())
        .prompt()
        .expect("msg");

    let _console = Select::new("Console Target:", get_consoles())
        .prompt()
        .expect("msg");

    genesis::create::project(_name.as_str());
}

fn get_project() -> Project {
    let project: Project = {
        let dir = current_dir().unwrap();
        let dir_value = dir.display();

        let project = std::fs::read_to_string(format!("{dir_value}/project.json"))
            .expect("Error Reading project.json");

        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<Project>(&project).unwrap()
    };
    return project;
}

fn print_logo() {
    println!("");
    println!("█▀█ █▀▀ ▀█▀ █▀█ █▀█ ▄▄ █▀▄ █▀▀ █░█ ▄▄ █░█ █▀▀ █░░ █▀█ █▀▀ █▀█");
    println!("█▀▄ ██▄ ░█░ █▀▄ █▄█ ░░ █▄▀ ██▄ ▀▄▀ ░░ █▀█ ██▄ █▄▄ █▀▀ ██▄ █▀▄");
    println!("");
}
