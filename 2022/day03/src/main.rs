use std::collections::HashSet;

fn priority(item: char) -> u32 {
    let mut raw = item as i32 - 96;
    if raw < 0 {
        raw += 58;
    }
    raw as u32
}

fn build_set(compartment: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    for c in compartment.chars() {
        set.insert(c);
    }
    set
}

fn priority_of_wrongly_packed(line: &str) -> u32 {
    let half_len = line.len() / 2;
    let left_compartment = &line[0..half_len];
    let right_compartment = &line[half_len..];
    let left_set = build_set(left_compartment);
    let right_set = build_set(right_compartment);

    let overlapping = left_set.intersection(&right_set);
    priority(*overlapping.last().expect("No overlapping values found!"))
}

fn priority_of_badges(raw: &str) -> u32 {
    let mut sum = 0;
    let mut line_iter = raw.lines();
    while let (Some(left), Some(middle), Some(right)) = (line_iter.next(), line_iter.next(), line_iter.next()) {
        let left_set = build_set(left);
        let middle_set = build_set(middle);
        let right_set = build_set(right);
        let overlapping: HashSet<char> = left_set.intersection(&middle_set).map(|x| *x).collect();
        let overlapping = overlapping.intersection(&right_set);
        sum += priority(*overlapping.last().expect("No overlapping values found!"))
    }
    sum
}

fn main() {
    let input = include_str!("../input.txt");
    let result_a: u32 = input.lines().map(priority_of_wrongly_packed).sum();
    let result_b = priority_of_badges(input);
    println!("A: {}", result_a);
    println!("B: {}", result_b);
}
