mod instruction;
pub use instruction::Instruction;

pub fn read_line(s: &str) -> Instruction {
    let mut iter = s.split_whitespace();
    let instruction = iter.next();
    // There must be no instruction on this line so skip it.
    if instruction.is_none() {
        return Instruction::None;
    }
    let mut args = vec![];
    iter.for_each(|arg| args.push(arg.to_string()));
    Instruction::parse(instruction.unwrap(), args)
}
