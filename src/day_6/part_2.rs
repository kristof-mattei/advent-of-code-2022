use std::collections::HashMap;

use crate::utils::read_file;

fn count_of_questions_answered_by_everybody(group: &[String]) -> u32 {
    let mut count_of_answers: HashMap<char, u32> = HashMap::new();

    for line in group {
        let mut duplicate_answer_per_line_check: Vec<char> = line.chars().into_iter().collect();
        duplicate_answer_per_line_check.sort();
        duplicate_answer_per_line_check.dedup();

        assert_eq!(duplicate_answer_per_line_check.len(), line.len());

        duplicate_answer_per_line_check.into_iter().for_each(|c| {
            let count = *(count_of_answers.get(&c).unwrap_or(&0));

            let _ = count_of_answers.insert(c, count + 1);
        });
    }

    let mut total_answers_that_everybody_answered: u32 = 0;
    let people_in_group: u32 = group.len().try_into().unwrap();

    for (_, value) in count_of_answers {
        if value == people_in_group {
            total_answers_that_everybody_answered += 1;
        }
    }

    total_answers_that_everybody_answered
}

fn parse_lines_into_questionaires(lines: &[String]) -> Vec<u32> {
    let mut all_answers: Vec<u32> = Vec::new();

    let groups = lines.split(String::is_empty);

    for group in groups {
        let unique_answers_in_group = count_of_questions_answered_by_everybody(group);

        all_answers.push(unique_answers_in_group);
    }

    all_answers
}

// https://adventofcode.com/2020/day/6
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_6/input.txt".into())?;

    let answers: u32 = parse_lines_into_questionaires(&split)
        .into_iter()
        .sum::<u32>()
        .try_into()
        .unwrap();

    Ok(answers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(3305, find_solution().unwrap());
    }

    #[test]
    fn answer_set_1() {
        let answer_set = ["abc".to_string()];

        assert_eq!(count_of_questions_answered_by_everybody(&answer_set), 3);
    }

    #[test]
    fn answer_set_2() {
        let answer_set = ["a".to_string(), "b".to_string(), "c".to_string()];

        assert_eq!(count_of_questions_answered_by_everybody(&answer_set), 0);
    }

    #[test]
    fn answer_set_3() {
        let answer_set = ["ab".to_string(), "ac".to_string()];

        assert_eq!(count_of_questions_answered_by_everybody(&answer_set), 1);
    }

    #[test]
    fn answer_set_4() {
        let answer_set = [
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
        ];

        assert_eq!(count_of_questions_answered_by_everybody(&answer_set), 1);
    }

    #[test]
    fn answer_set_5() {
        let answer_set = ["b".to_string()];

        assert_eq!(count_of_questions_answered_by_everybody(&answer_set), 1);
    }
}
