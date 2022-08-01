use std::path::PathBuf;

#[derive(Debug)]
pub enum Instruction {
    COPY(PathBuf, PathBuf),
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
            _ => panic!("Invalid instruction {}", instruction),
        }
    }
}
