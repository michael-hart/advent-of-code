#[cfg(windows)]
const DOUBLE_NEWLINE: &str = "\r\n\r\n";
#[cfg(not(windows))]
const DOUBLE_NEWLINE: &str = "\n\n";

#[derive(Debug)]
struct Passport {
    birth_year: Option<u64>,
    issue_year: Option<u64>,
    expiration_year: Option<u64>,
    height: Option<String>,
    hair_colour: Option<String>,
    eye_colour: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn empty() -> Passport {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_colour: None,
            eye_colour: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn is_valid(&self) -> bool {
        self.birth_year.is_some() &&
        self.issue_year.is_some() &&
        self.expiration_year.is_some() &&
        self.height.is_some() &&
        self.hair_colour.is_some() &&
        self.eye_colour.is_some() &&
        self.passport_id.is_some()
    }

    fn parse(raw: &str) -> Passport {
        let mut passport = Passport::empty();
        for token in raw.split_ascii_whitespace() {
            let parts : Vec<&str> = token.split(":").collect();
            let key = parts.get(0).unwrap();
            let val = parts.get(1).unwrap();
            match key {
                &"ecl" => passport.eye_colour = Some(String::from(*val)),
                &"pid" => passport.passport_id = Some(String::from(*val)),
                &"eyr" => passport.expiration_year = (*val).parse().ok(),
                &"hcl" => passport.hair_colour = Some(String::from(*val)),
                &"byr" => passport.birth_year = (*val).parse().ok(),
                &"iyr" => passport.issue_year = (*val).parse().ok(),
                &"cid" => passport.country_id = Some(String::from(*val)),
                &"hgt" => passport.height = Some(String::from(*val)),
                _ => (),
            }
        }

        passport
    }

}

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
    fn check_valid() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
        assert!(Passport::parse(input).is_valid());
    }

    #[test]
    fn check_valid_with_newlines() {
        let input = "ecl:gry pid:860033327 eyr:2020
hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
        assert!(Passport::parse(input).is_valid());
    }

    #[test]
    fn check_valid_missing_cid() {
        let input = "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm";
        assert!(Passport::parse(input).is_valid());
    }

    #[test]
    fn check_invalid_gives_none() {
        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929";
        assert!(!Passport::parse(input).is_valid());
    }

    #[test]
    fn check_batch_gives_correct_count() {
        let result = count_valid_passports_in_batch(include_str!("../test.txt"));
        assert_eq!(result, 2);
    }
}