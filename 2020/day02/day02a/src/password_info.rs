pub struct PasswordInfo {
    min_occurrences: u32,
    max_occurrences: u32,
    required_letter: char,
    password: String,
}

impl From<&str> for PasswordInfo {
    fn from(raw: &str) -> Self {
        let (min, max, req, pass) = scan_fmt!(raw.trim(), "{}-{} {}: {}", u32, u32, char, String).unwrap();
        PasswordInfo {
            min_occurrences: min,
            max_occurrences: max,
            required_letter: req,
            password: pass
        }
    }
}

impl PasswordInfo {
    pub fn is_valid(&self) -> bool {
        let count = self.password.chars()
            .filter(|x| *x == self.required_letter)
            .count() as u32;
        self.min_occurrences <= count && count <= self.max_occurrences
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gives_correct_response() {
        let parsed = PasswordInfo::from("  1-3 a: testing  \n");
        assert_eq!(parsed.min_occurrences, 1);
        assert_eq!(parsed.max_occurrences, 3);
        assert_eq!(parsed.required_letter, 'a');
        assert_eq!(parsed.password, "testing");
    }

    #[test]
    fn test_valid_pass_gives_valid() {
        assert!(PasswordInfo::from("1-3 a: tasting").is_valid());
        assert!(PasswordInfo::from("1-3 a: tastanag").is_valid());
    }

    #[test]
    fn test_invalid_pass_gives_invalid() {
        assert!(!PasswordInfo::from("1-3 a: tastanaga").is_valid());
        assert!(!PasswordInfo::from("1-3 a: testing").is_valid());
    }

}