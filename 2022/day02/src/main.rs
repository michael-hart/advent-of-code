fn parse_left_right(line: &str) -> (char, char) {
    let mut chars = line.chars();
    let left = chars.next().expect("No move for opponent!");
    let right = chars.skip(1).next().expect("No move for us!");
    (left, right)
}

fn score_a(line: &str) -> u32 {
    let (left, right) = parse_left_right(line);

    match right {
        'X' => 1 + match left {
            'A' => 3,  // Draw
            'C' => 6,  // Rock beats scissors
            'B' => 0,  // Rock loses to paper
            _ => 0,
        },
        'Y' => 2 + match left {
            'A' => 6, // Paper beats rock
            'B' => 3, // Draw
            'C' => 0, // Paper loses to scissors
            _ => 0,
        },
        'Z' => 3 + match left {
            'B' => 6, // Scissors beats paper
            'C' => 3, // Draw
            'A' => 0, // Scissors loses to rock
            _ => 0,
        },
        _ => 0,
    }
}

fn score_b(line: &str) -> u32 {
    let (left, right) = parse_left_right(line);
    match right {
        // Lose!
        'X' => match left {
            'A' => 3,  // Rock, lose with scissors, score of 3
            'B' => 1,  // Paper, lose with rock, score of 1
            'C' => 2,  // Scissors, lose with paper, score of 2
            _ => 0,
        },
        // Draw
        'Y' => 3 + match left {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => 0,
        },
        // Win
        'Z' => 6 + match left {
            'A' => 2, // Rock, win with paper, score of 2
            'B' => 3, // Paper, win with scissors, score of 3
            'C' => 1, // Scissors, win with rock, score of 1
            _ => 0, // Scissors loses to rock
            
        },
        _ => 0,
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let result_a: u32 = input.lines().map(score_a).sum();
    let result_b: u32 = input.lines().map(score_b).sum();
    println!("A: {}", result_a);
    println!("B: {}", result_b);
}
