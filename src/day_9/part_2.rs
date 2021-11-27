use crate::{shared::AoCResult, utils::read_file};

fn find_contiguous_set_of_numbers_that_sum_up_to(input: &[u64], target: u64) -> (u64, u64) {
    let mut offset = 0;
    let mut to_take = 2;

    loop {
        let range: Vec<&u64> = input.iter().skip(offset).take(to_take).collect();

        let sum: u64 = range.iter().copied().sum();

        match target.cmp(&sum) {
            std::cmp::Ordering::Less => {
                offset += 1;
                to_take = 2;
            }
            std::cmp::Ordering::Equal => {
                let min = *range.iter().min().unwrap();
                let max = *range.iter().max().unwrap();
                return (*min, *max);
            }
            std::cmp::Ordering::Greater => {
                to_take += 1;
            }
        }
    }
}

// https://adventofcode.com/2020/day/9
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    const TARGET: u64 = 138_879_426; // from day 9 part 1;
    let split = read_file("./src/day_9/input.txt".into())?;
    let input: Vec<u64> = split.iter().map(|s| s.parse::<u64>().unwrap()).collect();

    let (lowest, highest) = find_contiguous_set_of_numbers_that_sum_up_to(&input, TARGET);
    Ok(AoCResult::Ofu64(lowest + highest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(AoCResult::Ofu64(23761694), find_solution().unwrap());
    }

    #[test]
    fn test_example() {
        let input: Vec<u64> = vec![
            "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
            "127", "219", "299", "277", "309", "576",
        ]
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

        let (lowest, highest) = find_contiguous_set_of_numbers_that_sum_up_to(&input, 127);

        assert_eq!(15, lowest);
        assert_eq!(47, highest);
    }
}
