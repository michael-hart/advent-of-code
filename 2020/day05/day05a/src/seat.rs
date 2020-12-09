#[derive(Debug, PartialEq, Eq)]
pub struct Seat {
    row: u32,
    col: u32,
}


// BBF RRR
// u = 72
// l = 65
// hd = 4


impl From<&str> for Seat {
    fn from(raw: &str) -> Seat {
        let row_raw : String = raw.chars().take(7).collect();
        let col_raw : String = raw.chars().skip(7).collect();

        Seat::new(Seat::partition(&row_raw), Seat::partition(&col_raw))
    }
}

impl Seat {
    fn new(row: u32, col: u32) -> Seat {
        Seat { row, col }
    }
    pub fn seat_id(&self) -> u32 {
        self.row * 8 + self.col
    }

    fn partition(raw: &String) -> u32 {
        let mut num = 0;
        for c in raw.chars() {
            num *= 2;
            if c == 'B' || c == 'R' {
                num += 1;
            }
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_seat_from_str_correct() {
        assert_eq!(Seat::from("BFFFBBFRRR"), Seat::new(70, 7));
        assert_eq!(Seat::from("FFFBBBFRRR"), Seat::new(14, 7));
        assert_eq!(Seat::from("BBFFBBFRLL"), Seat::new(102, 4));
    }

    #[test]
    fn check_seat_ids() {
        assert_eq!(Seat::new(70, 7).seat_id(), 567);
        assert_eq!(Seat::new(14, 7).seat_id(), 119);
        assert_eq!(Seat::new(102, 4).seat_id(), 820);
    }
}