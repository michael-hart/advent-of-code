#[macro_use]
extern crate scan_fmt;

mod password_info;

use crate::password_info::PasswordInfo;


fn check_lines(raw: &str) -> u32 {
    raw.lines()
        .filter(|x| PasswordInfo::from(*x).is_valid())
        .count() as u32
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{} valid passwords", check_lines(input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_input_is_correct() {
        let small_input = include_str!("../test.txt");
        assert_eq!(check_lines(small_input), 2);
    }
}
