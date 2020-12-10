use crate::instruction_type::InstructionType;

#[derive(Debug)]
pub struct Instruction {
    pub instr: InstructionType,
    pub executed: bool,
}

impl From<&str> for Instruction {
    fn from(raw: &str) -> Self {
        Self { instr: InstructionType::from(raw), executed: false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop_correct() {
        let result = Instruction::from("nop +0");
        assert_eq!(result.instr, InstructionType::Nop(0));
        assert_eq!(result.executed, false);
    }

    #[test]
    fn test_acc_correct() {
        let result = Instruction::from("acc -3");
        assert_eq!(result.instr, InstructionType::Acc(-3));
        assert_eq!(result.executed, false);
    }

    #[test]
    fn test_jmp_correct() {
        let result = Instruction::from("jmp +4");
        assert_eq!(result.instr, InstructionType::Jump(4));
        assert_eq!(result.executed, false);
    }
}
