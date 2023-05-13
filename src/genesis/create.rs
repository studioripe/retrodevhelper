use std::{fs, process::exit};

use inquire::Select;
use rust_i18n::t;
use serde_derive::Serialize;

#[derive(Serialize)]
struct Project {
    name: String,
    sdk: String,
}

pub fn project(project_name: &str) {
    let ans = Select::new(t!("sdk").as_str(), get_genesis_sdks()).prompt();

    match ans {
        Ok(_choice) => {
            let sample = r#"#include <genesis.h>

int main()
{
    VDP_drawText("Hello World!", 10,13);
    while(1)
    {
        SYS_doVBlankProcess();
    }
    return (0);
}"#;

            println!("{}", t!("creating_project").as_str());
            let _dir = fs::create_dir(project_name);
            let data = Project {
                name: project_name.to_string(),
                sdk: _choice.to_string(),
            };

            let _dir = fs::create_dir(format!("{project_name}/out"));
            let _dir = fs::create_dir(format!("{project_name}/res"));
            let _dir = fs::create_dir(format!("{project_name}/src"));

            fs::write(format!("{project_name}/src/main.c"), sample)
                .expect(t!("file_write_error").as_str());

            let _j = match serde_json::to_string_pretty(&data) {
                Ok(v) => fs::write(format!("{project_name}/project.json"), v)
                    .expect(t!("file_write_error").as_str()),
                Err(_) => {
                    // Write `msg` to `stderr`.
                    eprintln!("{}", t!("error_creating_json").as_str());
                    // Exit the program with exit code `1`.
                    exit(1);
                }
            };
            println!("{}", t!("project_created").as_str());
        }
        Err(_) => println!("{}", t!("error_selecting").as_str()),
    }
}

fn get_genesis_sdks() -> Vec<&'static str> {
    vec!["SGDK"]
}
