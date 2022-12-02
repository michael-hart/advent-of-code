use std::collections::HashMap;

fn try_get_words_of_length(words: &Vec<String>, len: usize) -> Vec<&String> {
    words
        .iter()
        .filter(|w| w.len() == len)
        .collect()
}

fn contains(outer: &String, inner: &String) -> bool {
    inner.chars().all(|c| outer.contains(c))
}

fn solve_output(inputs: &Vec<String>, outputs: &Vec<String>) -> u32 {
    let mut codes = HashMap::new();

    // Find 1,4,7,8 due to unique word lengths
    let one = try_get_words_of_length(inputs, 2).get(0).expect("No word of length 2!").clone();
    codes.insert(one, 1);
    codes.insert(try_get_words_of_length(inputs, 4).get(0).expect("No word of length 4!").clone(), 4);
    codes.insert(try_get_words_of_length(inputs, 3).get(0).expect("No word of length 3!").clone(), 7);
    codes.insert(try_get_words_of_length(inputs, 7).get(0).expect("No word of length 7!").clone(), 8);

    // Find 3 as the only word of length 5 that contains 1's characters
    let fives = try_get_words_of_length(inputs, 5);
    let three = fives
        .iter()
        .filter(|w| contains(w, one))
        .nth(0)
        .expect("Could not find 3!")
        .clone();
    codes.insert(three, 3);

    // Find 9 as the only word of length 6 that contains 3's characters
    let sixes = try_get_words_of_length(inputs, 6);
    let nine = sixes
        .iter()
        .filter(|w| contains(w, three))
        .nth(0)
        .expect("Could not find 9!")
        .clone();
    codes.insert(nine, 9);

    // Find 5 as the length 5 words, minus 3, that fits in 9
    let five = fives
        .iter()
        .filter(|w| w != &&three && contains(nine, w))
        .nth(0)
        .expect("Could not find 5!")
        .clone();
    codes.insert(five, 5);

    // 2 is the remaining length 5
    let two = fives
        .iter()
        .filter(|w| w != &&three && w != &&five)
        .nth(0)
        .expect("Could not find 2!")
        .clone();
    codes.insert(two, 2);

    // 6 is the length 6 that contains 5
    let six = sixes
        .iter()
        .filter(|w| w != &&nine && contains(w, five))
        .nth(0)
        .expect("Could not find 6!")
        .clone();
    codes.insert(six, 6);

    // 0 is the remaining length 6
    let zero = sixes
        .iter()
        .filter(|w| w != &&nine && w != &&six)
        .nth(0)
        .expect("Could not find 0!")
        .clone();
    codes.insert(zero, 0);

    // Now decode the output
    outputs
        .iter()
        .map(|w| codes.get(w).expect("Did not recognise keyword!"))
        .fold(0, |acc, x| acc * 10 + x)
}

fn solve_raw(line: &str) -> u32 {
    let idx = line.chars().position(|c| c == '|').expect("Must have | in line!");
    let in_slice = &line[..idx];
    let out_slice = &line[idx + 1 ..];

    let converter = |line: &str| {
        line
            .split(" ")
            .filter(|w| w.len() > 0)
            .map(|w| {
                let mut chars: Vec<char> = w.chars().collect();
                chars.sort_by(|a, b| a.cmp(b));
                String::from_iter(chars)
            })
            .collect::<Vec<String>>()
    };

    let inputs = converter(in_slice);
    let outputs = converter(out_slice);

    solve_output(&inputs, &outputs)
}

fn part_a(raw: &str) -> usize {
    raw
        .lines()
        .map(|l| {
            let idx = l.chars().position(|c| c == '|').expect("Must have | in line!");
            let slice = &l[idx + 1 ..];
            let codes = slice.split(" ").filter(|w| w.len() > 0).collect::<Vec<&str>>();
            codes
        })
        .flatten()
        .filter(|w| w.len() == 2 || w.len() == 3 || w.len() == 4 || w.len() == 7)
        .count()
}

fn part_b(raw: &str) -> u32 {
    raw
        .lines()
        .map(|l| solve_raw(l))
        .fold(0, |acc, x| acc + x)
}

fn main() {
    let input = include_str!("../data/input.txt");
    
    println!("Part A: {}", part_a(input));
    println!("Part B: {}", part_b(input));
}
