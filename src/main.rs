#![warn(unused_variables, dead_code)]
extern crate simplelog;

use simplelog::*;

use clap::{arg, Command};
mod files;
mod utils;
use std::env;

fn cli() -> Command {
    Command::new("kms")
        .about("Rust adaptation of windows and linux commands.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("ls").about("Display files in directory"))
        .subcommand(
            Command::new("touch")
                .about("Create new file")
                .arg(arg!(<NAME> "Name of the file"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("mkdir")
                .about("Create new directory")
                .arg(arg!(<NAME> "Name of the directory"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("rmdir")
                .about("Remove directory")
                .arg(arg!(<NAME> "Name of the directory"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("del")
                .about("Remove file")
                .arg(arg!(<NAME> "Name of the file"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("cat")
                .about("Read file")
                .arg(arg!(<NAME> "Name of the file"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("tail")
                .about("Get 10 last lines of file")
                .arg(arg!(<NAME> "Name of the file"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();
    let current_path = env::current_dir().unwrap();

    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    match matches.subcommand() {
        Some(("ls", sub_matches)) => {
            info!("Directory {}", current_path.display());
            files::list_dir(current_path.to_str().unwrap());
        }
        Some(("touch", sub_matches)) => {
            let file_name = sub_matches
                .get_one::<String>("NAME")
                .map(|s| s.as_str())
                .expect("file name is required");

            if file_name.is_empty() {
                return error!("File name is required.");
            }

            files::create_file(file_name)
        }
        Some(("mkdir", sub_matches)) => {
            let dir_name = sub_matches
                .get_one::<String>("NAME")
                .map(|s| s.as_str())
                .expect("directory name is required");

            if dir_name.is_empty() {
                return error!("Directory name is required.");
            }

            files::create_dir(dir_name)
        }
        Some(("rmdir", sub_matches)) => {
            let dir_name = sub_matches
                .get_one::<String>("NAME")
                .map(|s| s.as_str())
                .expect("directory name is required");

            if dir_name.is_empty() {
                return error!("Directory name is required.");
            }

            files::remove_dir(dir_name)
        }
        Some(("del", sub_matches)) => {
            let dir_name = sub_matches
                .get_one::<String>("NAME")
                .map(|s| s.as_str())
                .expect("file name is required");

            if dir_name.is_empty() {
                return error!("File name is required.");
            }

            files::remove_file(dir_name)
        }
        Some(("cat", sub_matches)) => {
            let file_name = sub_matches
                .get_one::<String>("NAME")
                .map(|s| s.as_str())
                .expect("file name is required");

            if file_name.is_empty() {
                return error!("File name is required.");
            }

            files::read_file(file_name)
        }
        Some(("tail", sub_matches)) => {
            let file_name = sub_matches
                .get_one::<String>("NAME")
                .map(|s| s.as_str())
                .expect("file name is required");

            if file_name.is_empty() {
                return error!("File name is required.");
            }

            files::tail_file(file_name)
        }
        None => {}
        _ => unreachable!(),
    }
}
