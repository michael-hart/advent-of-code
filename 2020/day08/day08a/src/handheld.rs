use crate::instruction::Instruction;
use crate::instruction_type::InstructionType;

pub struct Handheld {
    instructions: Vec<Instruction>,
}

impl From<&str> for Handheld {
    fn from(raw: &str) -> Self {
        Handheld { instructions: raw.lines().map(Instruction::from).collect() }
    }
}

impl Handheld {
    pub fn get_accum_at_loop_restart(&mut self) -> i32 {
        let mut current = 0;
        let mut acc = 0;

        loop {
            let current_instruction = self.instructions.get_mut(current).unwrap();
            if current_instruction.executed {
                return acc;
            }

            match current_instruction.instr {
                InstructionType::Nop(_) => current += 1,
                InstructionType::Acc(n) => { current += 1; acc += n; },
                InstructionType::Jump(n) => current = ((current as i32) + n) as usize,
            }

            current_instruction.executed = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_gives_correct_result() {
        assert_eq!(Handheld::from(include_str!("../test.txt")).get_accum_at_loop_restart(), 5);
        assert!(true);
    }
}
