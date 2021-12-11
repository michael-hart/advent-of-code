use std::cmp::{min, max};
use std::collections::HashMap;

fn diff(a: u32, b: u32) -> u32 {
    max(a, b) - min(a, b)
}

fn diff_increasing(a: u32, b: u32) -> u32 {
    (1..=diff(a, b))
        .fold(0, |acc, x| acc + x)
}

fn mean(crabs: &[u32]) -> u32 {
    let sum = crabs.iter().fold(0, |acc, x| acc + x);
    let average = sum / crabs.len() as u32;
    average
}

fn fuel_needed(dest: u32, crabs: &[u32], diff_func: fn(a: u32, b: u32) -> u32) -> u32 {
    crabs
        .iter()
        .map(|x| diff_func(*x, dest))
        .fold(0, |acc, x| acc + x)
}

fn next_guess(guess: u32, guesses: &mut HashMap<u32, u32>, crabs: &[u32], diff_func: fn(a: u32, b: u32) -> u32) -> u32 {
    for x in guess - 2 .. guess + 3 {
        if guesses.contains_key(&x) {
            continue;
        }
        guesses.insert(x, fuel_needed(x, crabs, diff_func));
    }

    guesses
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| *k)
        .unwrap()
}

fn part_a(crabs: &[u32]) -> u32 {
    let mut guess = mean(&crabs);
    let mut fuels = HashMap::new();

    loop {
        let next = next_guess(guess, &mut fuels, &crabs, diff);
        if next == guess {
            break;
        }
        guess = next;
    }

    *fuels.get(&guess).unwrap()
}

fn part_b(crabs: &[u32]) -> u32 {
    let mut guess = mean(&crabs);
    let mut fuels = HashMap::new();

    loop {
        let next = next_guess(guess, &mut fuels, &crabs, diff_increasing);
        if next == guess {
            break;
        }
        guess = next;
    }

    *fuels.get(&guess).unwrap()
}

fn main() {
    let input = include_str!("../data/input.txt");
    let crabs: Vec<u32> = input
        .split(",")
        .filter_map(|c| c.trim().parse().ok())
        .collect();
    println!("Part A: {}", part_a(&crabs));
    println!("Part B: {}", part_b(&crabs));
}
