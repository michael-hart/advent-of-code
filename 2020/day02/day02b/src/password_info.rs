pub struct PasswordInfo {
    left_index: usize,
    right_index: usize,
    required_letter: u8,
    password: String,
}

impl From<&str> for PasswordInfo {
    fn from(raw: &str) -> Self {
        let (left, right, req, pass) = scan_fmt!(raw.trim(), "{}-{} {}: {}", usize, usize, char, String).unwrap();
        PasswordInfo {
            left_index: left,
            right_index: right,
            required_letter: req as u8,
            password: pass
        }
    }
}

impl PasswordInfo {
    pub fn is_valid(&self) -> bool {
        let left_valid = *self.password.as_bytes().get(self.left_index - 1).unwrap() == self.required_letter;
        let right_valid = *self.password.as_bytes().get(self.right_index - 1).unwrap() == self.required_letter;
        left_valid ^ right_valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gives_correct_response() {
        let parsed = PasswordInfo::from("  1-3 a: testing  \n");
        assert_eq!(parsed.left_index, 1);
        assert_eq!(parsed.right_index, 3);
        assert_eq!(parsed.required_letter, 'a' as u8);
        assert_eq!(parsed.password, "testing");
    }

    #[test]
    fn test_valid_pass_gives_valid() {
        assert!(PasswordInfo::from("1-3 a: abcde").is_valid());
    }

    #[test]
    fn test_invalid_pass_gives_invalid() {
        assert!(!PasswordInfo::from("1-3 b: cdefg").is_valid());
        assert!(!PasswordInfo::from("2-9 c: ccccccccc").is_valid());
    }

}