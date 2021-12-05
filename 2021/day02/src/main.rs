mod part_a;
mod part_b;
mod types;

use part_a::part_a;
use part_b::part_b;
use types::FromStr;

fn main() {
    let instructions: Vec<types::Instruction> = include_str!("../data/input.txt")
        .lines()
        .filter_map(|l| types::Instruction::from_str(l).ok())
        .collect();
    println!("Part A: {}", part_a(&instructions));
    println!("Part B: {}", part_b(&instructions));
}
