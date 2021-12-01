fn part_a(input: &Vec<u32>) -> u32 {
    input
        .iter()
        .zip(input.iter().skip(1))
        .fold(0, |acc, (x, y)| if y > x { acc + 1 } else { acc } )
}

fn part_b(input: &Vec<u32>) -> u32 {
    let sums = input
        .iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2))
        .map( |((x, y), z)| x + y + z)
        .collect();
    part_a(&sums)
}

fn main() {
    let input_raw = include_str!("../data/input.txt");
    let input: Vec<u32> = input_raw
        .lines()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("Part A: {}", part_a(&input));
    println!("Part B: {}", part_b(&input));
}
