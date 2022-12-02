fn is_open(c: char) -> bool {
    c == '<' || c == '(' || c == '{' || c == '['
}

fn is_close(c: char) -> bool {
    c == '>' || c == ')' || c == '}' || c == ']'
}

fn line_corrupt(line: &str) -> Option<char> {
    let mut stack = vec![];
    for c in line.chars() {
        if is_open(c) {
            stack.push(c);
        }
        if is_close(c) {
            if let Some(last) = stack.last() {
                // If doesn't match opening, corrupt
                let mut matches = false;
                match last {
                    '<' => { if c == '>' { matches = true; } },
                    '(' => { if c == ')' { matches = true; } },
                    '{' => { if c == '}' { matches = true; } },
                    '[' => { if c == ']' { matches = true; } },
                    _ => (),
                }
                if !matches {
                    return Some(c);
                } else {
                    stack.pop();
                }
            } else {
                // Trying to close with no open; corrupt
                return Some(c);
            }
        }
    }
    None
}

fn syntax_highscore(line: &str) -> usize {
    match line_corrupt(line) {
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        Some(_) | None => 0,
    }
}

fn part_a(code: &str) -> usize {
    code
        .lines()
        .map(|l| syntax_highscore(l))
        .fold(0, |acc, x| acc + x)
}

fn autocomplete_score(c: &char) -> usize {
    // Match on the opening bracket, rather than deciding on the closing bracket and assigning score to that
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn unclosed_brackets(line: &str) -> Vec<char> {
    let mut stack = vec![];
    for c in line.chars() {
        if is_open(c) {
            stack.push(c);
        }
        if is_close(c) {
            stack.pop();
        }
    }
    stack
}

fn line_autocomplete_score(unclosed: &Vec<char>) -> usize {
    unclosed
        .iter()
        .rev()
        .fold(0, |acc, x| acc * 5 + autocomplete_score(x))
}

fn part_b(code: &str) -> usize {
    let mut scores: Vec<usize> = code
        .lines()
        .filter(|l| line_corrupt(l).is_none())
        .map(unclosed_brackets)
        .map(|v| line_autocomplete_score(&v))
        .collect();
    scores.sort();
    *scores.get(scores.len() / 2).expect("Could not take middle score!")
}

fn main() {
    let code = include_str!("../data/input.txt");
    println!("Part A: {}", part_a(code));
    println!("Part B: {}", part_b(code));
}
