use std::collections::HashMap;

use crate::utils::read_file;

struct RuleWithPassword {
    min: u32,
    max: u32,
    character: char,
    password: String,
}

impl RuleWithPassword {
    pub fn is_valid(&self) -> bool {
        let mut counts: HashMap<char, u32> = HashMap::new();

        self.password.chars().for_each(|c| {
            let count = *(counts.get(&c).unwrap_or(&0));

            let _ = counts.insert(c, count + 1);
        });

        match counts.get(&self.character) {
            None => false,
            Some(t) => (self.min <= *t) && (*t <= self.max),
        }
    }
}

fn parse_line(line: &str) -> RuleWithPassword {
    // grammar:
    // <min>-<max> char: <password>
    let pieces: Vec<&str> = line.split(' ').collect();

    let min_max_vec: Vec<u32> = pieces[0]
        .split('-')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let char_with_colon_piece = pieces[1];
    let password = pieces[2];

    RuleWithPassword {
        min: *min_max_vec.get(0).unwrap(),
        max: *min_max_vec.get(1).unwrap(),
        character: char_with_colon_piece.chars().nth(0).unwrap(),
        password: password.into(),
    }
}

// https://adventofcode.com/2020/day/2
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_2/input.txt".into())?;

    let valid_passwords = split
        .into_iter()
        .map(|s| parse_line(&s))
        .filter(|rwp| rwp.is_valid())
        .count();

    Ok(valid_passwords as u32)
}

#[test]
fn outcome() {
    assert_eq!(620, find_solution().unwrap());
}
