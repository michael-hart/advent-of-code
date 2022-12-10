fn is_unique(recent: &[char]) -> bool {
    if recent.len() == 1 {
        return true;
    }
    let match_count = recent.iter().skip(1).filter(|x| **x == recent[0]).count();
    match_count == 0 && is_unique(&recent[1..])
}

fn marker_idx(text: &str, unique_len: usize) -> usize {
    let mut recent_chars = text.chars().take(unique_len).collect::<Vec<char>>();
    for (idx, c) in text.chars().enumerate().skip(unique_len) {
        if is_unique(recent_chars.as_slice()) {
            return idx;
        }
        recent_chars = recent_chars
            .iter()
            .skip(1)
            .take(unique_len - 1)
            .chain(vec![c].iter())
            .map(|x| *x)
            .collect();
    }

    0
}

fn main() {
    let raw = include_str!("../input.txt").trim();
    println!("A: {}", marker_idx(raw, 4));
    println!("B: {}", marker_idx(raw, 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_4() {
        let marker = marker_idx("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4);
        assert_eq!(marker, 7);
    }

    #[test]
    fn test_second_4() {
        let marker = marker_idx("bvwbjplbgvbhsrlpgdmjqwftvncz", 4);
        assert_eq!(marker, 5);
    }

    #[test]
    fn test_first_14() {
        let marker = marker_idx("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14);
        assert_eq!(marker, 19);
    }
}