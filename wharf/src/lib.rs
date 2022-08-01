use std::{fs, path::PathBuf, process::Command};

mod lang;
use colored::Colorize;
use lang::Instruction::{self, *};

pub fn run(path: PathBuf) {
    parse(path)
        .iter()
        .for_each(|instruction| match instruction {
            COPY(src, dst) => {
                if src.is_dir() {
                    copy_dir(src, dst);
                } else {
                    fs::copy(src, dst).expect("Failed to copy the file");
                }
            }
            CMD(cmd, args) => {
                let mut command = Command::new("cmd.exe");
                command.args(&["/C", cmd]);
                args.iter().for_each(|arg| {
                    if !arg.eq("--ignore") {
                        command.arg(arg);
                    }
                });
                command.spawn().expect("Failed to execute the command");
            }
            OWN(_) => {}
            PRINT(s) => {
                println!("{}", s);
            }
            None => {}
        });
}

pub fn reverse(path: PathBuf) {
    parse(path)
        .iter()
        .rev()
        .for_each(|instruction| match instruction {
            COPY(src, dst) => {
                if src.is_dir() {
                    fs::remove_dir_all(dst).expect("Failed to remove the directory");
                } else {
                    fs::remove_file(dst).expect("Failed to remove the file");
                }
            }
            CMD(cmd, args) => {
                let mut cmd_string = cmd.to_owned();
                args.iter().for_each(|arg| {
                        cmd_string.push_str(" ");
                        cmd_string.push_str(arg);
                });
                if !cmd_string.contains("--ignore") {
                    println!("{}: External commands cannot be reversed, if you are the developer, we strongly recommend you use the provided commands that are reversable, or use the OWN command to tell wharf the outputs of the command ({})", "Warning".yellow(), cmd_string)
                }
            }
            OWN(path) => {
                if path.is_dir() {
                    fs::remove_dir_all(path).expect("Failed to remove the directory");
                } else {
                    fs::remove_file(path).expect("Failed to remove the file");
                }
            }
            // Cannot reverse a print
            PRINT(_) => {}
            None => {}
        })
}

fn parse(path: PathBuf) -> Vec<Instruction> {
    let contents = fs::read_to_string(path).expect("Failed to read the file");
    let instructions_strs = contents.lines();
    instructions_strs
        .filter(|line| line.trim().len() > 0)
        .filter(|line| !line.starts_with("#"))
        .filter(|line| !line.starts_with("//"))
        .map(|line| lang::read_line(line))
        .collect()
}

// Helper function to copy directories
fn copy_dir(src: &PathBuf, dst: &PathBuf) {
    fs::create_dir_all(dst).expect("Failed to create the directory");
    for entry in fs::read_dir(src).expect("Failed to read the directory") {
        let entry = entry.expect("Failed to read the entry");
        let path = entry.path();
        let dst_path = dst.join(path.file_name().unwrap());
        if path.is_dir() {
            copy_dir(&path, &dst_path);
        } else {
            fs::copy(&path, &dst_path).expect("Failed to copy the file");
        }
    }
}
