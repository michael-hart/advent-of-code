use std::collections::HashSet;

#[cfg(windows)]
const DOUBLE_NEWLINE: &str = "\r\n\r\n";
#[cfg(not(windows))]
const DOUBLE_NEWLINE: &str = "\n\n"; 

fn count_group(raw: &str) -> u32 {
    let mut set = HashSet::new();
    for line in raw.lines() {
        for c in line.chars() {
            set.insert(c);
        }
    }
    set.len() as u32
}

fn count_sum(raw: &str) -> u32 {
    raw
        .split(DOUBLE_NEWLINE)
        .map(count_group)
        .sum()
}

fn main() {
    println!("Sum of counts is {}", count_sum(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_count_sum_correct() {
        assert_eq!(count_sum(include_str!("../test.txt")), 11);
    }

}
