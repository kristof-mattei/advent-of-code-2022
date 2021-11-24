use crate::{shared::AoCResult, utils::read_file};

fn parse_group_of_answers(group: &[String]) -> u32 {
    let mut answers: Vec<char> = Vec::new();

    for line in group {
        line.chars().for_each(|p| answers.push(p));
    }

    answers.sort_unstable();
    answers.dedup();

    answers.len().try_into().unwrap()
}

fn parse_lines_into_questionaires(lines: &[String]) -> Vec<u32> {
    let mut all_answers: Vec<u32> = Vec::new();

    let groups = lines.split(String::is_empty);

    for group in groups {
        let unique_answers_in_group = parse_group_of_answers(group);

        all_answers.push(unique_answers_in_group);
    }

    all_answers
}

// https://adventofcode.com/2020/day/6
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_6/input.txt".into())?;

    let answers: u32 = parse_lines_into_questionaires(&split)
        .into_iter()
        .sum::<u32>();

    Ok(AoCResult::Ofu32(answers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(AoCResult::Ofu32(6521), find_solution().unwrap());
    }

    #[test]
    fn answer_set_1() {
        let answer_set = ["abc".to_string()];

        assert_eq!(parse_group_of_answers(&answer_set), 3);
    }

    #[test]
    fn answer_set_2() {
        let answer_set = ["a".to_string(), "b".to_string(), "c".to_string()];

        assert_eq!(parse_group_of_answers(&answer_set), 3);
    }

    #[test]
    fn answer_set_3() {
        let answer_set = ["ab".to_string(), "ac".to_string()];

        assert_eq!(parse_group_of_answers(&answer_set), 3);
    }

    #[test]
    fn answer_set_4() {
        let answer_set = [
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
        ];

        assert_eq!(parse_group_of_answers(&answer_set), 1);
    }

    #[test]
    fn answer_set_5() {
        let answer_set = ["b".to_string()];

        assert_eq!(parse_group_of_answers(&answer_set), 1);
    }
}
