use crate::utils::read_file;

#[derive(Default, Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn is_valid_pid(passport_id: &str) -> bool {
    if passport_id.len() != 9 {
        return false;
    }

    let mut passport_id_chars = passport_id.chars();

    for _ in 0..9 {
        match passport_id_chars.next() {
            Some(c) if ('0'..='9').contains(&c) => (),
            _ => return false, // short circuit
        }
    }

    true
}

fn is_valid_hcl(haircolor: &str) -> bool {
    if haircolor.len() != 7 {
        return false;
    }

    let mut haircolor_chars = haircolor.chars();

    if haircolor_chars.next() != Some('#') {
        return false;
    }

    for _ in 0..6 {
        match haircolor_chars.next() {
            Some(c) if ('0'..='9').contains(&c) || ('a'..='f').contains(&c) => (),
            _ => return false, // short circuit
        }
    }

    true
}

fn is_valid_height(
    height_as_string: &str,
    suffix: &str,
    minimum_inclusive: u32,
    maximum_inclusive: u32,
) -> bool {
    if height_as_string.ends_with(suffix) {
        matches!(height_as_string[0..(height_as_string.len() - suffix.len())].parse::<u32>(), Ok(height) if height >= minimum_inclusive && height <= maximum_inclusive)
    } else {
        false
    }
}

impl Passport {
    fn is_byr_valid(&self) -> bool {
        match &self.byr {
            Some(byr) => matches!(byr.parse::<u32>(), Ok(b) if (1920..=2002).contains(&b)),
            _ => false,
        }
    }
    fn is_iyr_valid(&self) -> bool {
        match &self.iyr {
            Some(iyr) => matches!(iyr.parse::<u32>(), Ok(i) if (2010..=2020).contains(&i)),
            _ => false,
        }
    }
    fn is_eyr_valid(&self) -> bool {
        match &self.eyr {
            Some(eyr) => matches!(eyr.parse::<u32>(), Ok(e) if (2020..=2030).contains(&e)),
            _ => false,
        }
    }

    fn is_hgt_valid(&self) -> bool {
        match &self.hgt {
            Some(hgt) => is_valid_height(hgt, "cm", 150, 193) || is_valid_height(hgt, "in", 59, 76),
            None => false,
        }
    }
    fn is_hcl_valid(&self) -> bool {
        match &self.hcl {
            Some(hcl) => is_valid_hcl(hcl),
            _ => false,
        }
    }
    fn is_ecl_valid(&self) -> bool {
        match &self.ecl {
            Some(ecl) => {
                let valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

                valid_colors.contains(&&**ecl)
            }
            None => false,
        }
    }

    fn is_pid_valid(&self) -> bool {
        match &self.pid {
            Some(pid) => is_valid_pid(pid),
            None => false,
        }
    }

    // fn is_cid_valid(&self) -> bool {
    //     true // shhh
    // }
    fn is_valid(&self) -> bool {
        self.is_byr_valid()
            && self.is_iyr_valid()
            && self.is_eyr_valid()
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.is_ecl_valid()
            && self.is_pid_valid()
        //&& self.is_cid_valid()
    }
}

fn parse_passport(passport_pieces: &[&str]) -> Passport {
    let mut passport: Passport = Passport::default();

    for piece in passport_pieces {
        let split: Vec<&str> = piece.split(':').collect();

        let identifier = split[0];
        let value = split[1];

        match identifier {
            "byr" /*(Birth Year)*/ => passport.byr = Some(value.into()),
            "iyr" /*(Issue Year)*/ => passport.iyr = Some(value.into()),
            "eyr" /*(Expiration Year)*/ => passport.eyr = Some(value.into()),
            "hgt" /*(Height)*/ => passport.hgt = Some(value.into()),
            "hcl" /*(Hair Color)*/ => passport.hcl = Some(value.into()),
            "ecl" /*(Eye Color)*/ => passport.ecl = Some(value.into()),
            "pid" /*(Passport ID)*/ => passport.pid = Some(value.into()),
            "cid" /*(Country ID)*/ => passport.cid = Some(value.into()),
            _ => panic!("WTF DID YOU GIVE ME?")
        }
    }

    passport
}

fn parse_line_group(line_group: &[String]) -> Passport {
    let mut passport_pieces = Vec::new();

    for line in line_group {
        line.split(' ').for_each(|p| passport_pieces.push(p));
    }

    parse_passport(&passport_pieces)
}

fn parse_lines_into_groups(lines: &[String]) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();

    let groups = lines.split(String::is_empty);

    for group in groups {
        let passport = parse_line_group(group);

        passports.push(passport);
    }

    passports
}

// https://adventofcode.com/2020/day/4
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_4/input.txt".into())?;

    let valid_passports = parse_lines_into_groups(&split)
        .into_iter()
        .filter(Passport::is_valid)
        .count();

    Ok(valid_passports.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(116, find_solution().unwrap());
    }

    #[test]
    fn test_byr_2002_valid() {
        let passport = Passport {
            byr: Some("2002".to_string()),
            ..Passport::default()
        };

        assert!(passport.is_byr_valid());
    }

    #[test]
    fn test_byr_2003_invalid() {
        let passport = Passport {
            byr: Some("2003".to_string()),
            ..Passport::default()
        };

        assert!(!passport.is_byr_valid());
    }

    #[test]
    fn test_hgt_60in_valid() {
        let passport = Passport {
            hgt: Some("60in".to_string()),
            ..Passport::default()
        };

        assert!(passport.is_hgt_valid());
    }

    #[test]
    fn test_hgt_190cm_valid() {
        let passport = Passport {
            hgt: Some("190cm".to_string()),
            ..Passport::default()
        };

        assert!(passport.is_hgt_valid());
    }

    #[test]
    fn test_hgt_190in_invalid() {
        let passport = Passport {
            hgt: Some("190in".to_string()),
            ..Passport::default()
        };

        assert!(!passport.is_hgt_valid());
    }

    #[test]
    fn test_hgt_190_invalid() {
        let passport = Passport {
            hgt: Some("190".to_string()),
            ..Passport::default()
        };

        assert!(!passport.is_hgt_valid());
    }

    #[test]
    fn test_hcl_pound123abc_valid() {
        let passport = Passport {
            hcl: Some("#123abc".to_string()),
            ..Passport::default()
        };

        assert!(passport.is_hcl_valid());
    }

    #[test]
    fn test_hcl_pound123abz_invalid() {
        let passport = Passport {
            hcl: Some("#123abz".to_string()),
            ..Passport::default()
        };

        assert!(!passport.is_hcl_valid());
    }
    #[test]
    fn test_hcl_123abz_invalid() {
        let passport = Passport {
            hcl: Some("123abz".to_string()),
            ..Passport::default()
        };

        assert!(!passport.is_hcl_valid());
    }

    #[test]
    fn test_ecl_brn_valid() {
        let passport = Passport {
            ecl: Some("brn".to_string()),
            ..Passport::default()
        };

        assert!(passport.is_ecl_valid());
    }

    #[test]
    fn test_ecl_wat_invalid() {
        let passport = Passport {
            ecl: Some("wat".to_string()),
            ..Passport::default()
        };

        assert!(!passport.is_ecl_valid());
    }

    #[test]
    fn test_pid_000000001_valid() {
        let passport = Passport {
            pid: Some("000000001".to_string()),
            ..Passport::default()
        };

        assert!(passport.is_pid_valid());
    }

    #[test]
    fn test_pid_0123456789_invalid() {
        let passport = Passport {
            pid: Some("0123456789".into()),
            ..Passport::default()
        };

        assert!(!passport.is_pid_valid());
    }

    fn arr_of_amp_str_to_vec_of_string(arr: &[&str]) -> Vec<String> {
        arr.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn is_valid_passport_1() {
        let raw_lines = [
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
        ];

        assert!(parse_line_group(&arr_of_amp_str_to_vec_of_string(&raw_lines)).is_valid());
    }

    #[test]
    fn is_valid_passport_2() {
        let raw_lines = [
            "eyr:2029 ecl:blu cid:129 byr:1989",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        ];
        assert!(parse_line_group(&arr_of_amp_str_to_vec_of_string(&raw_lines)).is_valid());
    }

    #[test]
    fn is_valid_passport_3() {
        let raw_lines = [
            "hcl:#888785",
            "hgt:164cm byr:2001 iyr:2015 cid:88",
            "pid:545766238 ecl:hzl",
            "eyr:2022",
        ];
        assert!(parse_line_group(&arr_of_amp_str_to_vec_of_string(&raw_lines)).is_valid());
    }

    #[test]
    fn is_valid_passport_4() {
        let raw_lines = ["iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"];
        assert!(parse_line_group(&arr_of_amp_str_to_vec_of_string(&raw_lines)).is_valid());
    }

    #[test]
    fn is_invalid_passport_1() {
        let raw_lines = [
            "eyr:1972 cid:100",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        ];
        assert!(!parse_line_group(&arr_of_amp_str_to_vec_of_string(&raw_lines)).is_valid());
    }

    #[test]
    fn is_invalid_passport_2() {
        let raw_lines = [
            "iyr:2019",
            "hcl:#602927 eyr:1967 hgt:170cm",
            "ecl:grn pid:012533040 byr:1946",
        ];
        assert!(!parse_line_group(&arr_of_amp_str_to_vec_of_string(&raw_lines)).is_valid());
    }
    #[test]
    fn is_invalid_passport_3() {
        let raw_lines = [
            "hcl:dab227 iyr:2012",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        ];
        assert!(!parse_line_group(&arr_of_amp_str_to_vec_of_string(&raw_lines)).is_valid());
    }
    #[test]
    fn is_invalid_passport_4() {
        let raw_lines = [
            "hgt:59cm ecl:zzz",
            "eyr:2038 hcl:74454a iyr:2023",
            "pid:3556412378 byr:2007",
        ];
        assert!(!parse_line_group(&arr_of_amp_str_to_vec_of_string(&raw_lines)).is_valid());
    }
}
