use crate::{shared::AoCResult, utils::read_file};

fn calculate_permutations(input: &[u32]) -> u64 {
    let mut copy: Vec<u32> = vec![0];
    copy.append(&mut input.iter().copied().collect());

    copy.sort_unstable();

    copy.push(input.iter().max().unwrap() + 3);

    println!("Max index {}", input.len());
    calculate_permutations_r(&copy, 0)
}

fn calculate_permutations_r(input: &[u32], index: usize) -> u64 {
    let current = input.get(index).unwrap();

    let mut possible_permutations: u64 = 0;

    if input.get(index + 1).is_none() {
        // end of the line, valid
        return 1;
    }

    for i in 1..=3 {
        let next_possible_index = index + i;

        match input.get(next_possible_index) {
            Some(next) => {
                if next - current <= 3 {
                    possible_permutations += calculate_permutations_r(input, next_possible_index);
                }
            }
            None => (),
        }
    }

    println!("possible_permutations: {}", possible_permutations);

    possible_permutations
}

// https://adventofcode.com/2020/day/9
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_10/input.txt".into())?;
    let input: Vec<u32> = split.iter().map(|s| s.parse::<u32>().unwrap()).collect();

    let permutations = calculate_permutations(&input);
    Ok(AoCResult::Ofu64(permutations))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(AoCResult::Ofu32(1820), find_solution().unwrap());
    }

    #[test]
    fn test_example_1() {
        let input: Vec<u32> = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"]
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let permutations = calculate_permutations(&input);

        assert_eq!(8, permutations);
    }

    #[test]
    fn test_example_2() {
        let input: Vec<u32> = vec![
            "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45",
            "19", "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34",
            "10", "3",
        ]
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

        let permutations = calculate_permutations(&input);

        assert_eq!(19208, permutations);
    }
}
