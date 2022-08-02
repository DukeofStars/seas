use std::path::PathBuf;

#[derive(Debug)]
pub enum Instruction {
    /// Copies something from one location to another.
    COPY(PathBuf, PathBuf),
    /// Executes an external command.
    CMD(String, Vec<String>),
    /// Prints a message to the console.
    PRINT(String),
    /// "Owns" a folder or file, allowing it to be removed when the script is reversed.
    OWN(PathBuf),
    /// Attaches another "rope" to this one.
    ATTACH(String),
    None,
}

impl Instruction {
    pub fn parse(instruction: &str, args: Vec<String>) -> Self {
        match instruction.trim().to_uppercase().as_str() {
            "COPY" => Instruction::COPY(
                args.get(0)
                    .expect("Expected argument")
                    .parse()
                    .expect("Expected Path"),
                args.get(1)
                    .expect("Expected argument")
                    .parse()
                    .expect("Expected Path"),
            ),
            "CMD" => Instruction::CMD(
                args.get(0).expect("Expected command").to_owned(),
                args.split_first().unwrap().1.to_vec(),
            ),
            "OWN" => Instruction::OWN(
                args.get(0)
                    .expect("Expected argument")
                    .parse()
                    .expect("Expected Path"),
            ),
            "PRINT" => Instruction::PRINT(args.join(" ")),
            "ATTACH" => Instruction::ATTACH(args.get(0).expect("Expected rope name").to_owned()),
            _ => panic!("Invalid instruction {}", instruction),
        }
    }
}
