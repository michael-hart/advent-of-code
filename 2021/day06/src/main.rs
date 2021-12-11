use std::collections::HashMap;

fn tick(lanterns: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut next = HashMap::new();
    lanterns
        .iter()
        .for_each(|(ls, c)| {
            if *ls == 0 {
                *next.entry(6).or_insert(0) += *c;
                next.insert(8, *c);
            } else {
                *next.entry(ls - 1).or_insert(0) += *c;
            }
        });
    next
}

fn part_a(lanterns: &HashMap<usize, usize>) -> usize {
    (0..80)
        .fold(lanterns.clone(), |acc, _| tick(&acc))
        .iter()
        .fold(0, |acc, (_, x)| acc + *x)
}

fn part_b(lanterns: &HashMap<usize, usize>) -> usize {
    (0..256)
        .fold(lanterns.clone(), |acc, _| tick(&acc))
        .iter()
        .fold(0, |acc, (_, x)| acc + *x)
}

fn main() {
    let input = include_str!("../data/input.txt");
    let lanternfish: Vec<usize> = input
        .split(",")
        .filter_map(|c| c.trim().parse().ok())
        .collect();
    let mut lanternfish_counts = HashMap::new();
    lanternfish
        .iter()
        .for_each(|l| *lanternfish_counts.entry(*l).or_insert(0) += 1);

    println!("Part A: {}", part_a(&lanternfish_counts));
    println!("Part B: {}", part_b(&lanternfish_counts));
}
