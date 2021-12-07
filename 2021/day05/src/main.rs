use std::cmp::{min, max};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 4,5
        let comma_idx = match s.chars().position(|c| c == ',') {
            Some(x) => x,
            None => return Err("Input string not of correct format!".to_string()),
        };
        let x = *match &s[..comma_idx].parse::<u32>() {
            Ok(x) => x,
            Err(_) => return Err("Could not parse x!".to_string()),
        };
        let y = *match &s[comma_idx + 1 ..].parse::<u32>() {
            Ok(x) => x,
            Err(_) => return Err("Could not parse y!".to_string()),
        };
        Ok(Point { x, y })
    }
}

fn make_line(line: &str, accept_diagonals: bool) -> Result<Vec<Point>, String> {
    // 4,5 -> 5,9
    let dash_idx = match line.chars().position(|c| c == '-') {
        Some(x) => x,
        None => return Err("Input string not of correct format!".to_string()),
    };
    let start = Point::from_str(&line[..dash_idx - 1]).unwrap();
    let end = Point::from_str(&line[dash_idx + 3 ..]).unwrap();

    if !accept_diagonals && start.x != end.x && start.y != end.y {
        Err("Diagonal lines not accepted!".to_string())
    } else if accept_diagonals && start.x != end.x && start.y != end.y {
        // (1,2); (3, 0) -> (1,2), (2, 1), (3, 0)
        let x_iter: Vec<u32> = if start.x > end.x {
            (end.x ..= start.x).rev().collect()
        } else {
            (start.x ..= end.x).collect()
        };
        let y_iter: Vec<u32> = if start.y > end.y {
            (end.y ..= start.y).rev().collect()
        } else {
            (start.y ..= end.y).collect()
        };
        Ok(
            x_iter
                .iter()
                .zip(y_iter.iter())
                .map(|(x, y)| Point { x: *x, y: *y })
                .collect()
        )
    } else if start.x != end.x {
        Ok(
            (min(start.x, end.x) ..= max(start.x, end.x))
                .map(|x| Point { x, y: start.y })
                .collect()
        )
    } else {
        Ok(
            (min(start.y, end.y) ..= max(start.y, end.y))
                .map(|y| Point { x: start.x, y })
                .collect()
        )
    }
}

fn part_a(input: &str) -> usize {
    let mut map: HashMap<Point, u32> = HashMap::new();
    input
        .lines()
        .filter_map(|line| make_line(line, false).ok())
        .flatten()
        .for_each(|p| *map.entry(p).or_insert(0) += 1);
    map
        .iter()
        .filter(|(_, c)| **c > 1)
        .count()
}

fn part_b(input: &str) -> usize {
    let mut map: HashMap<Point, u32> = HashMap::new();
    input
        .lines()
        .filter_map(|line| make_line(line, true).ok())
        .flatten()
        .for_each(|p| *map.entry(p).or_insert(0) += 1);
    map
        .iter()
        .filter(|(_, c)| **c > 1)
        .count()
}

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Part A: {}", part_a(input));
    println!("Part B: {}", part_b(input));
}
