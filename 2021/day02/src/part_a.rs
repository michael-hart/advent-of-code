use super::types::{Direction, Instruction};

#[derive(Debug)]
struct Position {
    forward: u32,
    depth: u32,
}

impl Position {
    fn from(forward: u32, depth: u32) -> Position {
        Position { forward, depth }
    }

    fn next_position(&self, instruction: &Instruction) -> Position {
        match instruction.dir {
            Direction::Forward => Position::from(self.forward + instruction.mag, self.depth),
            Direction::Up => Position::from(self.forward, self.depth - instruction.mag),
            Direction::Down => Position::from(self.forward, self.depth + instruction.mag),
        }
    }

    fn product(&self) -> u32 {
        self.forward * self.depth
    }
}

pub fn part_a(instructions: &Vec<Instruction>) -> u32 {
    instructions.iter()
        .fold(Position::from(0, 0), |acc, x| acc.next_position(&x))
        .product()
}
