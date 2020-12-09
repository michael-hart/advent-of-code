mod passport;

use passport::Passport;

#[cfg(windows)]
const DOUBLE_NEWLINE: &str = "\r\n\r\n";
#[cfg(not(windows))]
const DOUBLE_NEWLINE: &str = "\n\n";

fn count_valid_passports_in_batch(raw: &str) -> usize {
    raw
        .split(DOUBLE_NEWLINE)
        .map(Passport::parse)
        .filter(Passport::is_valid)
        .count()
}

fn main() {
    println!("{} valid passports", count_valid_passports_in_batch(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_batch_gives_correct_count() {
        let result = count_valid_passports_in_batch(include_str!("../test.txt"));
        assert_eq!(result, 2);
    }
}