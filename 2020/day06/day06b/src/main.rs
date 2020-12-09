use std::collections::HashSet;

#[cfg(windows)]
const DOUBLE_NEWLINE: &str = "\r\n\r\n";
#[cfg(not(windows))]
const DOUBLE_NEWLINE: &str = "\n\n"; 

fn merge_sets(left: &HashSet<char>, right: &HashSet<char>) -> HashSet<char> {
    let mut output = HashSet::new();

    for c in left.iter() {
        if right.contains(c) {
            output.insert(*c);
        }
    }
    
    output
}

fn count_group(raw: &str) -> u32 {
    let mut set = HashSet::new();
    let mut line_iter = raw.lines();

    for c in line_iter.next().unwrap().chars() {
        set.insert(c);
    }

    for line in line_iter {
        let mut current = HashSet::new();
        for c in line.chars() {
            current.insert(c);
        }
        set = merge_sets(&set, &current);
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
        assert_eq!(count_sum(include_str!("../test.txt")), 6);
    }

}
