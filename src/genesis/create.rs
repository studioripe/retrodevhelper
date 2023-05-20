use std::{collections::HashMap, env::current_dir, fs, path::Path, process::exit};

use directories::ProjectDirs;
use inquire::Select;
use rust_i18n::t;
use serde_derive::{Deserialize, Serialize};
use spinoff::{spinners, Color, Spinner};

#[derive(Serialize)]
struct Project {
    name: String,
    sdk: String,
}

pub fn project(project_name: &str) {
    let ans = Select::new(t!("sdk").as_str(), get_genesis_sdks()).prompt();

    match ans {
        Ok(_choice) => {
            if let Some(proj_dirs) = ProjectDirs::from("com", "StudioRipe", "RetroDevHelper") {
                let data = proj_dirs.data_local_dir().to_str().expect("msg");

                if !Path::new(&format!("{data}/SGDK")).exists() {
                    let spinner = Spinner::new(spinners::Dots, t!("download_sgdk"), Color::Blue);

                    git_download::repo("https://github.com/Stephane-D/SGDK")
                        .branch_name("master")
                        .add_file("res/", format!("{data}/SGDK/res"))
                        .add_file("inc/", format!("{data}/SGDK/inc"))
                        .exec()
                        .expect("FAIL");
                    spinner.success(t!("download_sgdk_complete").as_str())
                }

                if !Path::new(&format!("{data}/templates")).exists() {
                    git_download::repo("https://github.com/studioripe/retrodevhelper")
                        .branch_name("main")
                        .add_file("templates/", format!("{data}/templates"))
                        .exec()
                        .expect("FAIL");
                }
            }

            let spinner = Spinner::new(spinners::Dots, t!("creating_project"), Color::Blue);

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
            spinner.success(t!("project_created").as_str());
        }
        Err(_) => println!("{}", t!("error_selecting").as_str()),
    }
}

fn get_genesis_sdks() -> Vec<&'static str> {
    vec!["SGDK"]
}

fn replace_cpp_values() {
    let config: CppConfig = {
        let dir = current_dir().unwrap();
        let dir_value = dir.display();

        let config = std::fs::read_to_string(format!("{dir_value}/.vscode/c_cpp_properties.json"))
            .expect(t!("project_error").as_str());

        serde_json::from_str::<CppConfig>(&config).unwrap()
    };

    println!("{}", &config.configurations[0].includePath.join(" "));

    let _j = match serde_json::to_string_pretty(&config) {
        Ok(v) => fs::write(format!(".vscode/c_cpp_properties.json"), v)
            .expect(t!("file_write_error").as_str()),
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("{}", t!("error_creating_json").as_str());
        }
    };
}

#[derive(Serialize, Debug, Deserialize)]
struct CppConfig {
    #[serde(flatten)]
    configurations: Vec<L1>,
    version: String,
}

#[derive(Serialize, Debug, Deserialize)]
struct L1 {
    name: String,
    includePath: Vec<String>,
    defines: Vec<String>,
    macFrameworkPath: Vec<String>,
    compilerPath: String,
    cStandard: String,
    cppStandard: String,
    intelliSenseMode: String,
}
