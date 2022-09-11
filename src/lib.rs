#![feature(fs_try_exists)]
use std::fs;
use std::env;
use std::io::Write;
use std::error::Error;

pub struct Config {
    pub proj_name: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let proj_name = args[1].clone();

        Ok(Config { proj_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if !fs::try_exists(&config.proj_name)? {
        // make project folder and change root
        fs::create_dir_all(&config.proj_name)?;
        env::set_current_dir(&config.proj_name)?;

        // make folders
        fs::create_dir_all("src")?;
        fs::create_dir_all("res")?;
        fs::create_dir_all("include")?;
        fs::create_dir_all("bin/debug")?;
        fs::create_dir_all("bin/release")?;
        fs::create_dir_all(".vscode")?;

        // make files
        let mut main_file = fs::File::create("src/main.cpp")?;
        main_file.write(
            b"#include <iostream>
        
int main()
{
     std::cout << \"Hello World!\" << std::endl;
}",
        )?;
        let mut tasks_file = fs::File::create(".vscode/tasks.json")?;
        tasks_file.write(
            b"{
            \"version\": \"2.0.0\",
            \"tasks\": [
                {
                    \"type\": \"cppbuild\",
                    \"label\": \"debug\",
            \"command\": \"g++\",
            \"args\": [
                \"${workspaceFolder}/src/*.cpp\",
                \"-o\",
                \"${workspaceFolder}/bin/debug/main\",
                \"-g\",
                \"-Wall\",
                \"-I\",
                \"include\"
            ]
        }
    ]
}",
        )?;
    } else {
        println!("Folder already exists!");
    }

    Ok(())
}
