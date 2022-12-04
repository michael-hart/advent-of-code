use std::ops::RangeInclusive;


fn either_contained<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
where T: std::fmt::Debug + PartialOrd
{
    range_contains(&a, &b) || range_contains(&b, &a)
}

fn range_contains<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
    where T: PartialOrd
{
    a.contains(&b.start()) && a.contains(&b.end())
}

fn any_overlap<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
    where T: PartialOrd
{
    a.contains(&b.start()) ||
        a.contains(&b.end()) ||
        b.contains(&a.start()) ||
        b.contains(&a.end())
}

fn parse_pair(pair: &str) -> RangeInclusive<i32> {
    let mut split = pair.split("-");
    let left = split.next().expect("No dash in input").parse::<i32>().expect("No number on left of dash");
    let right = split.next().expect("No dash in input").parse::<i32>().expect("No number on right of dash");
    left..=right
}

fn parse_line(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let mut split = line.split(",");
    let left = parse_pair(split.next().expect("No comma in input"));
    let right = parse_pair(split.next().expect("No comma in input"));

    (left, right)
}

fn main() {
    let raw = include_str!("../input.txt");
    let result_a = raw.lines()
        .map(parse_line)
        .map(|(a, b)| either_contained(&a, &b))
        .filter(|x| *x)
        .count();
    let result_b = raw.lines()
        .map(parse_line)
        .map(|(a, b)| any_overlap(&a, &b))
        .filter(|x| *x)
        .count();
    println!("A: {}", result_a);
    println!("B: {}", result_b);
}
