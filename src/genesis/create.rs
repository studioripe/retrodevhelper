use std::{
    env::{current_dir, set_current_dir},
    fs, io,
    path::Path,
    process::exit,
};

use directories::ProjectDirs;
use inquire::Select;
use rust_i18n::t;
use serde_derive::Serialize;
use serde_json::Value;
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
            if let Some(proj_dirs) = ProjectDirs::from("com", "studioripe", "retrodevhelper") {
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

                let spinner: Spinner =
                    Spinner::new(spinners::Dots, t!("creating_project"), Color::Blue);

                let _dir = fs::create_dir(project_name);

                copy_dir_all(format!("{data}/templates/sgdk"), format!("{project_name}/"))
                    .expect("Error");

                set_current_dir(format!("{project_name}/")).expect("Error");
                replace_cpp_values(format!("{data}/SGDK")).expect("Error Replacing CPP Values");

                let data = Project {
                    name: project_name.to_string(),
                    sdk: _choice.to_string(),
                };

                let _j = match serde_json::to_string_pretty(&data) {
                    Ok(v) => fs::write(format!("project.json"), v)
                        .expect(t!("file_write_error").as_str()),
                    Err(_) => {
                        eprintln!("{}", t!("error_creating_json").as_str());
                        exit(1);
                    }
                };

                spinner.success(t!("project_created").as_str());
            }
        }
        Err(_) => println!("{}", t!("error_selecting").as_str()),
    }
}

fn get_genesis_sdks() -> Vec<&'static str> {
    vec!["SGDK"]
}

fn replace_cpp_values(data_dir: String) -> io::Result<()> {
    let dir = current_dir().unwrap();
    let dir_value = dir.display();

    let mut config = std::fs::read_to_string(format!("{dir_value}/.vscode/c_cpp_properties.json"))?;

    let include_paths = [
        r##"${workspaceFolder}/**""##,
        &format!(r##""{data_dir}/inc""##),
        &format!(r##""{data_dir}/res"##),
    ];

    config = config.replace("${workspaceFolder}/**", include_paths.join(",").as_str());

    let value: Value = serde_json::from_str(&config).unwrap();

    let formatted = serde_json::to_string_pretty(&value)?;

    fs::write(format!(".vscode/c_cpp_properties.json"), formatted)?;

    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
