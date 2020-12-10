#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InstructionType {
    Nop(i32),
    Acc(i32),
    Jump(i32),
}

impl From<&str> for InstructionType {
    fn from(raw: &str) -> Self {
        let mut parts = raw.split_ascii_whitespace();
        let in_raw = parts.next().unwrap();
        let num = parts.next().unwrap().parse().unwrap();

        match in_raw {
            "nop" => InstructionType::Nop(num),
            "acc" => InstructionType::Acc(num),
            "jmp" => InstructionType::Jump(num),
            _ => panic!("Unrecognised instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_correct() {
        assert_eq!(InstructionType::from("nop +0"), InstructionType::Nop(0));
        assert_eq!(InstructionType::from("acc -3"), InstructionType::Acc(-3));
        assert_eq!(InstructionType::from("jmp +4"), InstructionType::Jump(4));
    }
}
