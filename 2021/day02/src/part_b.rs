use super::types::{Direction, Instruction};

#[derive(Debug)]
struct SubStatus {
    forward: u32,
    depth: u32,
    aim: u32,
}

impl Default for SubStatus {
    fn default() -> SubStatus {
        SubStatus::from(0, 0, 0)
    }
}

impl SubStatus {

    fn from(forward: u32, depth: u32, aim: u32) -> SubStatus {
        SubStatus { forward, depth, aim }
    }

    fn next_status(&self, instruction: &Instruction) -> SubStatus {
        match instruction.dir {
            Direction::Forward => SubStatus::from(
                self.forward + instruction.mag,
                self.depth + self.aim * instruction.mag,
                self.aim),
            Direction::Up => SubStatus::from(
                self.forward,
                self.depth,
                self.aim - instruction.mag),
            Direction::Down => SubStatus::from(
                self.forward,
                self.depth,
                self.aim + instruction.mag),
        }
    }

    fn product(&self) -> u32 {
        self.forward * self.depth
    }
}

pub fn part_b(instructions: &Vec<Instruction>) -> u32 {
    instructions.iter()
        .fold(SubStatus::default(), |acc, x| acc.next_status(&x))
        .product()
}
