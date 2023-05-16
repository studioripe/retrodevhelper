use std::{
    env::set_current_dir,
    fs,
    path::Path,
    process::{exit, Command},
};

use directories::ProjectDirs;
use inquire::Select;
use rust_i18n::t;
use serde_derive::Serialize;
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
            let spinner = Spinner::new(spinners::Dots, t!("creating_project"), Color::Blue);

            if let Some(proj_dirs) = ProjectDirs::from("com", "StudioRipe", "RetroDevHelper") {
                let data = proj_dirs.data_local_dir().to_str().expect("msg");

                if !Path::new(&format!("{data}/SGDK")).exists() {
                    println!("{}", t!("download_sgdk").as_str());
                    git_download::repo("https://github.com/Stephane-D/SGDK")
                        .branch_name("master")
                        .add_file("res/", format!("{data}/SGDK/res"))
                        .add_file("inc/", format!("{data}/SGDK/inc"))
                        .exec()
                        .expect("FAIL");
                }
            }

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
