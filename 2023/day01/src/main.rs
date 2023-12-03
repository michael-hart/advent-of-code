use once_cell::sync::Lazy;
use std::collections::HashMap;

static NUMBERS: Lazy<HashMap<&str, u32>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("one", 1);
    m.insert("two", 2);
    m.insert("three", 3);
    m.insert("four", 4);
    m.insert("five", 5);
    m.insert("six", 6);
    m.insert("seven", 7);
    m.insert("eight", 8);
    m.insert("nine", 9);
    m
});

fn calc_without_replacement(line: &str) -> u32 {
    let first = line.chars().find(char::is_ascii_digit).unwrap();
    let last = line.chars().rev().find(char::is_ascii_digit).unwrap();

    first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
}

fn calc_with_replacement(line: &str) -> u32 {
    let check_line_idx = |idx: usize, c: char, line: &str| {
        if let Some(value) = c.to_digit(10) {
            return Some(value);
        }

        for n in 3..6 {
            if line.len() - idx < n {
                return None;
            }
            let sub = &line[idx..idx+n];
            if let Some(value) = NUMBERS.get(sub) {
                return Some(*value);
            }
        }

        None
    };
    let first = line
        .chars()
        .enumerate()
        .find_map(|(idx, c)| check_line_idx(idx, c, line))
        .unwrap();
    let last = line
        .chars()
        .rev()
        .enumerate()
        .find_map(|(idx, c)| check_line_idx(line.len() - idx - 1, c, line))
        .unwrap();
    first * 10 + last
}

fn part_a(text: &str) -> u32 {
    text.lines().map(calc_without_replacement).sum()
}

fn part_b(text: &str) -> u32 {
    text.lines().map(calc_with_replacement).sum()
}

fn main() {
    let in_str = include_str!("../input.txt");
    println!("{}", part_a(in_str));
    println!("{}", part_b(in_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_digit() {
        assert_eq!(calc_without_replacement("abc1def"), 11);
    }

    #[test]
    fn test_many_digits() {
        assert_eq!(calc_without_replacement("a2bc1de3f"), 23);
    }

    #[test]
    fn test_digit_replacement() {
        assert_eq!(part_b("atwob2c3d4e5f"), 25);
        assert_eq!(part_b("two1nine"), 29);
        assert_eq!(part_b("eightwothree"), 83);
    }

    #[test]
    fn test_part_a() {
        let s = include_str!("../test_a.txt");
        assert_eq!(part_a(s), 142);
    }

    #[test]
    fn test_part_b() {
        let s = include_str!("../test_b.txt");
        assert_eq!(part_b(s), 281);
    }
}
