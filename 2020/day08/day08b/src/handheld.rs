use crate::instruction::Instruction;
use crate::instruction_type::InstructionType;

#[derive(Debug, Clone)]
pub struct Handheld {
    instructions: Vec<Instruction>,
}

// impl Copy for Handheld {
//     fn copy(other: &Self) -> Handheld {
//         Handheld { instructions : other.instructions.iter().map(Instruction::copy).collect() }
//     }
// }

impl From<&str> for Handheld {
    fn from(raw: &str) -> Self {
        Handheld { instructions: raw.lines().map(Instruction::from).collect() }
    }
}

impl Handheld {
    fn get_accum_if_terminated(&mut self) -> Option<i32> {
        let mut current = 0;
        let mut acc = 0;

        loop {
            if current >= self.instructions.len() {
                return Some(acc);
            }

            let current_instruction = self.instructions.get_mut(current).unwrap();
            if current_instruction.executed {
                return None;
            }

            match current_instruction.instr {
                InstructionType::Nop(_) => current += 1,
                InstructionType::Acc(n) => { current += 1; acc += n; },
                InstructionType::Jump(n) => current = ((current as i32) + n) as usize,
            }

            current_instruction.executed = true;
        }
    }

    pub fn get_accum_after_terminate(&self) -> Option<i32> {
        let handheld_clone = self.clone();
        let mut handheld_clone = handheld_clone;

        for idx in 0..self.instructions.len() {
            let instr = handheld_clone.instructions.get_mut(idx).unwrap();
            
            match instr.instr {
                InstructionType::Jump(n) => {
                    instr.instr = InstructionType::Nop(n);
                    let result = handheld_clone.get_accum_if_terminated();
                    if result.is_some() {
                        return result;
                    }
                    let tmp_clone = self.clone();
                    handheld_clone = tmp_clone;
                }
                _ => (),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_gives_correct_result() {
        assert_eq!(Handheld::from(include_str!("../test.txt")).get_accum_after_terminate(), Some(8));
        assert!(true);
    }
}
