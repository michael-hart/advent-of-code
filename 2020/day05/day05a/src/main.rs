mod seat;
use seat::Seat;

fn highest_seat_id(raw: &str) -> u32 {
    raw
        .lines()
        .map(|l| Seat::from(l).seat_id())
        .max()
        .unwrap()
}

fn main() {
    println!("Highest seat ID: {}", highest_seat_id(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_highest() {
        assert_eq!(highest_seat_id("BFFFBBFRRR\r\nFFFBBBFRRR\r\nBBFFBBFRLL\r\n"), 820);
    }
}
