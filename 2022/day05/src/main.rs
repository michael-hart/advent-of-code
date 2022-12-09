use std::fmt;

use lazy_static::lazy_static;
use regex::{Regex, Captures};

type Stack<T> = Vec<T>;

fn parse_stacks(raw: &str) -> Vec<Stack<char>> {
    let first_lines: Vec<&str> = raw.lines().take_while(|x| x.len() > 0).collect();

    // How many stacks are there?
    let num_stacks = first_lines
        .last()
        .expect("No last line available!")
        .trim()
        .split(" ")
        .last()
        .expect("No characters in string!")
        .parse::<i32>()
        .expect("Unable to parse integer to string!");

    let mut stacks: Vec<Stack<char>> = vec![];
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }

    for line in first_lines.iter().take(first_lines.len() - 1).rev() {
        for (raw_idx, stack) in stacks.iter_mut().enumerate() {
            let idx = raw_idx * 4 + 1;
            let stack_entry: char = line.chars().skip(idx).next().expect("No character found at index!");
            if stack_entry != ' ' {
                stack.push(stack_entry)
            }
        }
    }

    stacks
}



struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct InstructionError;

impl fmt::Display for InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid string to parse to instruction")
    }
}

impl TryFrom<&str> for Instruction {
    type Error = InstructionError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new("move (\\d+) from (\\d+) to (\\d+)").expect("Could not compile regex!");
        }
        
        let cap = RE.captures(value).ok_or(InstructionError)?;

        let parse_int = |cap: &Captures, idx| cap.get(idx).map_or("", |m| m.as_str()).parse::<usize>().map_err(|_| InstructionError);

        let count = parse_int(&cap, 1)?;
        let from = parse_int(&cap, 2)? - 1;
        let to = parse_int(&cap, 3)? - 1;
        Ok(Instruction {count, from, to})
    }
}

fn parse_instructions(raw: &str) -> Vec<Instruction> {
    raw
        .lines()
        .skip_while(|x| x.len() > 0)
        .skip(1)
        .take_while(|x| x.len() > 0)
        .filter_map(|text| Instruction::try_from(text).ok())
        .collect()
}

fn move_stacks_9000(stacks: &mut Vec<Stack<char>>, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        for _ in 0..instruction.count {
            let val = stacks.get_mut(instruction.from).expect("Could not find stack to move from").pop().expect("Cannot pop from empty stack!");
            stacks.get_mut(instruction.to).expect("Could not find stack to move to").push(val);
        }
    }
}

fn move_stacks_9001(stacks: &mut Vec<Stack<char>>, instructions: &Vec<Instruction>) {
    let mut temp_stack: Vec<char> = vec![];
    for instruction in instructions {
        for _ in 0..instruction.count {
            let val = stacks.get_mut(instruction.from).expect("Could not find stack to move from").pop().expect("Cannot pop from empty stack!");
            temp_stack.push(val);
        }
        while temp_stack.len() > 0 {
            let val = temp_stack.pop().expect("Cannot pop from empty stack!");
            stacks.get_mut(instruction.to).expect("Could not find stack to move to").push(val);
        }
    }
}

fn top(stacks: &Vec<Stack<char>>) -> String {
    let mut top_str = "".to_string();

    for stack in stacks {
        let top_item = stack.iter().last().expect("Stack is empty!");
        top_str += &top_item.to_string();
    }

    top_str
}

fn main() {
    let raw = include_str!("../input.txt");
    let mut a_stacks = parse_stacks(raw);
    let instructions = parse_instructions(raw);
    
    move_stacks_9000(&mut a_stacks, &instructions);
    println!("A: {}", top(&a_stacks));
    
    let mut b_stacks = parse_stacks(raw);
    move_stacks_9001(&mut b_stacks, &instructions);
    println!("B: {}", top(&b_stacks));
}
