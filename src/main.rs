use clap::{App, Arg, SubCommand};
use dialoguer::Confirm;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let matches = App::new("Rusty Unity")
        .version("0.1")
        .author("Bartłomiej T. Listwon <blistwon@pineapple.works>")
        .about("Tools to bootstrap Unity project")
        .subcommand(
            SubCommand::with_name("init")
                .about("Initializes Unity project directory with defaults")
                .arg(
                    Arg::with_name("PATH")
                        .help("Working directory")
                        .default_value(".")
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        let gitignore = include_bytes!("gitignore.txt");
        let gitattributes = include_bytes!("gitattributes.txt");

        let directory = matches.value_of("PATH").unwrap();
        fs::create_dir_all(format!("{}{}", directory, "/Assets"))?;

        let gi_path = format!("{}{}", directory, "/.gitignore");

        if Path::new(&gi_path).exists() {
            if Confirm::new()
                .with_prompt("Do you want to overwrite .gitignore?")
                .interact()
                .unwrap()
            {
                let mut gitignore_file = File::create(gi_path.to_owned())?;
                gitignore_file.write_all(gitignore)?;
            }
        } else {
            let mut gitignore_file = File::create(gi_path.to_owned())?;
            gitignore_file.write_all(gitignore)?;
        }

        let ga_path = format!("{}{}", directory, "/.gitattributes");

        if Path::new(&ga_path).exists() {
            if Confirm::new()
                .with_prompt("Do you want to overwrite .gitattributes?")
                .interact()
                .unwrap()
            {
                let mut gitattributes_file = File::create(ga_path)?;
                gitattributes_file.write_all(gitattributes)?;
            }
        } else {
            let mut gitattributes_file = File::create(ga_path)?;
            gitattributes_file.write_all(gitattributes)?;
        }
    }

    Ok(())
}
