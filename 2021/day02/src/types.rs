pub use std::str::FromStr;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug)]
pub struct Instruction {
    pub dir: Direction,
    pub mag: u32,
}

impl Instruction {
    pub fn from(dir: Direction, mag: u32) -> Instruction {
        Instruction { dir: dir, mag: mag }
    }

    // fn operate(&self, pos: &Position) -> Position {
    //     match self.dir {
    //         Direction::Forward => Position::from(pos.forward + self.mag, pos.depth),
    //         Direction::Up => Position::from(pos.forward, pos.depth - self.mag),
    //         Direction::Down => Position::from(pos.forward, pos.depth + self.mag),
    //     }
    // }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let space_idx = match value.chars().position(|c| c == ' ') {
            Some(x) => x,
            None => return Err("Input string not of correct format!".to_string()),
        };
        let dir = &value[..space_idx];
        let mag = *match &value[space_idx + 1 ..].parse::<u32>() {
            Ok(x) => x,
            Err(_) => return Err("Could not parse number of steps!".to_string()),
        };
        if dir == "up" {
            Ok(Instruction::from(Direction::Up, mag))
        } else if dir == "down" {
            Ok(Instruction::from(Direction::Down, mag))
        } else if dir == "forward" {
            Ok(Instruction::from(Direction::Forward, mag))
        } else {
            Err(format!("Direction not recognised: {}", dir))
        }
    }
}
