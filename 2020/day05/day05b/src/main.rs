mod seat;
use seat::Seat;

use std::collections::HashSet;

fn seat_id_list(raw: &str) -> HashSet<u32> {
    raw
        .lines()
        .map(|l| Seat::from(l).seat_id())
        .collect()
}

fn missing_seat(ids: &HashSet<u32>) -> Option<u32> {
    for n in ids {
        if ids.contains(&(*n + 2)) && !ids.contains(&(*n + 1)) {
            return Some(*n + 1);
        }
    }
    None
}

fn main() {
    println!("Missing seat ID: {:?}", missing_seat(&seat_id_list(include_str!("../input.txt"))));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_missing_simple_case() {
        let set = [1u32, 2u32, 3u32, 5u32, 6u32].iter().map(|i| *i).collect();
        assert_eq!(missing_seat(&set), Some(4));
    }

}
