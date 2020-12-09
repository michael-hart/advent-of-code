
#[derive(Debug)]
pub struct Passport {
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

    fn birth_year_is_valid(&self) -> bool {
        match self.birth_year {
            Some(year) => year >= 1920 && year <= 2002,
            None => false
        }
    }

    fn issue_year_is_valid(&self) -> bool {
        match self.issue_year {
            Some(year) => year >= 2010 && year <= 2020,
            None => false
        }
    }

    fn expiration_year_is_valid(&self) -> bool {
        match self.expiration_year {
            Some(year) => year >= 2020 && year <= 2030,
            None => false
        }
    }

    fn height_is_valid(&self) -> bool {
        match &self.height {
            None => false,
            Some(hgt) => {
                if hgt.len() < 4 { return false; }
                let hgt_num : u64 =  match hgt.chars().take(hgt.len() - 2).collect::<String>().parse() {
                    Ok(n) => n,
                    Err(_) => return false,
                };

                if hgt.ends_with("cm") {
                    return hgt_num >= 150 && hgt_num <= 193;
                } else if hgt.ends_with("in") {
                    return hgt_num >= 59 && hgt_num <= 76;
                } else {
                    return false;
                }
            }
        }
    }

    fn hair_colour_is_valid(&self) -> bool {
        match &self.hair_colour {
            None => false,
            Some(clr) => {
                clr.len() == 7 &&
                clr.chars().next().unwrap() == '#' &&
                clr.chars().skip(1).map(|c| c.is_ascii_hexdigit()).fold(true, |acc, x| acc && x)
            }
        }
    }

    fn eye_colour_is_valid(&self) -> bool {
        match &self.eye_colour {
            None => false,
            Some(clr) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&clr.as_str()),
        }
    }

    fn passport_id_is_valid(&self) -> bool {
        match &self.passport_id {
            None => false,
            Some(id) => {
                id.len() == 9 &&
                id.chars().map(char::is_numeric).fold(true, |acc, x| acc && x)
            },
        }
    }

    pub fn is_valid(&self) -> bool {
        self.birth_year_is_valid() &&
        self.issue_year_is_valid() &&
        self.expiration_year_is_valid() &&
        self.height_is_valid() &&
        self.hair_colour_is_valid() &&
        self.eye_colour_is_valid() &&
        self.passport_id_is_valid()
    }

    pub fn parse(raw: &str) -> Passport {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_birth_year() {
        assert!(Passport::parse("byr:2002").birth_year_is_valid());
        assert!(!Passport::parse("byr:2003").birth_year_is_valid());
    }

    #[test]
    fn check_height() {
        assert!(Passport::parse("hgt:60in").height_is_valid());
        assert!(Passport::parse("hgt:190cm").height_is_valid());
        assert!(!Passport::parse("hgt:190in").height_is_valid());
        assert!(!Passport::parse("hgt:190").height_is_valid());
    }

    #[test]
    fn check_hair_colour() {
        assert!(Passport::parse("hcl:#123abc").hair_colour_is_valid());
        assert!(!Passport::parse("hcl:#123abz").hair_colour_is_valid());
        assert!(!Passport::parse("hcl:123abc").hair_colour_is_valid());
    }

    #[test]
    fn check_eye_colour() {
        assert!(Passport::parse("ecl:brn").eye_colour_is_valid());
        assert!(!Passport::parse("ecl:wat").eye_colour_is_valid());
    }

    #[test]
    fn check_passport_id_valid() {
        assert!(Passport::parse("pid:000000001").passport_id_is_valid());
        assert!(!Passport::parse("pid:0123456789").passport_id_is_valid());
    }

    #[test]
    fn check_invalid() {
        assert!(!Passport::parse("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926").is_valid());
        assert!(!Passport::parse("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946").is_valid());
        assert!(!Passport::parse("hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277").is_valid());
        assert!(!Passport::parse("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007").is_valid());
    }

    #[test]
    fn check_valid() {
        assert!(Passport::parse("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f").is_valid());
        assert!(Passport::parse("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm").is_valid());
        assert!(Passport::parse("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022").is_valid());
        assert!(Passport::parse("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719").is_valid());
    }

    #[test]
    fn check_valid_with_newlines() {
        let input = "pid:087499704 hgt:74in ecl:grn
iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
        assert!(Passport::parse(input).is_valid());
    }

    #[test]
    fn check_valid_missing_cid() {
        let input = "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm";
        assert!(Passport::parse(input).is_valid());
    }

}