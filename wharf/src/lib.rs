use std::{fs, path::PathBuf};

mod lang;
use lang::Instruction::*;

pub fn install(path: PathBuf) {
    let contents = fs::read_to_string(path).expect("Failed to read the file");
    let instructions_strs = contents.lines();
    let mut instructions = vec![];
    instructions_strs
        .filter(|line| line.trim().len() > 0)
        .for_each(|line| instructions.push(lang::read_line(line)));

    instructions
        .iter()
        .for_each(|instruction| match instruction {
            COPY(src, dst) => {
                if src.is_dir() {
                    copy_dir(src, dst);
                } else {
                    fs::copy(src, dst).expect("Failed to copy the file");
                }
            }
            None => {}
        });
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
