use crate::utils;
use chrono::{DateTime, Local};
use colored::Colorize;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
extern crate simplelog;

use simplelog::*;

pub fn list_dir(path: &str) {
    let dir = Path::new(path);
    if dir.is_dir() {
        println!("{} is a directory containing:", path);
        println!(
            "    {:<23} | {:<10} | {:<10} | {:<10} | {:<20}",
            "Name".bold().underline(),
            "Type".bold().underline(),
            "Size".bold().underline(),
            "Extension".bold().underline(),
            "Last Modified".bold().underline()
        );
        let mut entries: Vec<_> = fs::read_dir(dir).unwrap().collect();
        entries.sort_by_key(|entry| entry.as_ref().unwrap().path());
        let mut count = 0;
        for entry in &entries {
            let entry = entry.as_ref().unwrap();
            let metadata = entry.metadata().unwrap();
            let file_type = metadata.file_type();
            if file_type.is_dir() {
                count += 1;
                let file_size = metadata.len();
                let last_modified: SystemTime = metadata.modified().unwrap();
                let datetime: DateTime<Local> = last_modified.into();
                let file_name = entry.file_name().to_string_lossy().green();
                let truncated_file_name = utils::truncate_string(&file_name, 20);
                if count == entries.len() {
                    println!(
                        "└── \u{1F4C1} {:<20} | {:<10} | {:<10} | {:<10} | {:<20}",
                        truncated_file_name,
                        "directory".yellow(),
                        utils::pretty_size(file_size).purple(),
                        "none".red(),
                        datetime.format("%Y-%m-%d %H:%M:%S").to_string().blue(),
                    );
                } else {
                    println!(
                        "├── \u{1F4C1} {:<20} | {:<10} | {:<10} | {:<10} | {:<20}",
                        truncated_file_name,
                        "directory".yellow(),
                        utils::pretty_size(file_size).purple(),
                        "none".red(),
                        datetime.format("%Y-%m-%d %H:%M:%S").to_string().blue(),
                    );
                }
            }
        }
        for entry in &entries {
            let entry = entry.as_ref().unwrap();
            let metadata = entry.metadata().unwrap();
            let file_type = metadata.file_type();
            if file_type.is_file() {
                count += 1;
                let file_size = metadata.len();
                let last_modified: SystemTime = metadata.modified().unwrap();
                let datetime: DateTime<Local> = last_modified.into();
                let file_name = entry.file_name().to_string_lossy().into_owned();
                let truncated_file_name = utils::truncate_string(&file_name, 20);
                let extension = entry
                    .path()
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                let full_extension = if extension.is_empty() {
                    "none".to_string()
                } else {
                    extension
                };
                if count == entries.len() {
                    println!(
                        "└── \u{1F4C4} {:<20} | {:<10} | {:<10} | {:<10} | {:<20}",
                        truncated_file_name,
                        "file".yellow(),
                        utils::pretty_size(file_size).purple(),
                        full_extension.red(),
                        datetime.format("%Y-%m-%d %H:%M:%S").to_string().blue(),
                    );
                } else {
                    println!(
                        "├── \u{1F4C4} {:<20} | {:<10} | {:<10} | {:<10} | {:<20}",
                        truncated_file_name,
                        "file".yellow(),
                        utils::pretty_size(file_size).purple(),
                        full_extension.red(),
                        datetime.format("%Y-%m-%d %H:%M:%S").to_string().blue(),
                    );
                }
            }
        }
    } else {
        println!("{} is not a directory", path);
    }
}

pub fn create_file(file_name: &str) {
    let current_dir = std::env::current_dir().unwrap();
    let file_path = current_dir.join(file_name);
    std::fs::File::create(file_path).unwrap();
    info!("Created file {}", file_name);
}

pub fn create_dir(dir_name: &str) {
    let current_dir = std::env::current_dir().unwrap();
    let dir_path = current_dir.join(dir_name);
    std::fs::create_dir_all(dir_path).unwrap();
    info!("Created directory {}", dir_name);
}

pub fn remove_dir(dir_name: &str) {
    let current_dir = std::env::current_dir().unwrap();
    let dir_path = current_dir.join(dir_name);
    if !dir_path.exists() {
        error!("{} does not exist", dir_name);
        return;
    }
    std::fs::remove_dir_all(dir_path).unwrap();
    info!("Removed directory {}", dir_name);
}

pub fn remove_file(file_name: &str) {
    let current_dir = std::env::current_dir().unwrap();
    let file_path = current_dir.join(file_name);
    std::fs::remove_file(file_path).unwrap();
    info!("Removed file {}", file_name);
}

pub fn read_file(file_name: &str) {
    let current_dir = std::env::current_dir().unwrap();
    let file_path = current_dir.join(file_name);
    if !file_path.exists() {
        error!("{} does not exist", file_name);
        return;
    }
    let contents = fs::read_to_string(file_path).unwrap();
    info!("{}", contents);
}

pub fn tail_file(file_name: &str) {
    let current_dir = std::env::current_dir().unwrap();
    let file_path = current_dir.join(file_name);
    if !file_path.exists() {
        error!("{} does not exist", file_name);
        return;
    }
    let contents = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut count = 0;
    for line in lines.clone() {
        count += 1;
        if count > lines.len() - 10 {
            println!("{}", line);
        }
    }
}
