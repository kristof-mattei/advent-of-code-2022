use crate::{shared::AoCResult, utils::read_file};
fn slide_until_sum_of_any_2_in_last_25_is_not_current_value(input: &[i32]) -> i32 {
    todo!()
}
// https://adventofcode.com/2020/day/9
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    let _split = read_file("./src/day_9/input.txt".into())?;

    Ok(AoCResult::Ofi32(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(AoCResult::Ofi32(0), find_solution().unwrap());
    }

    #[test]
    fn test_example() {
        let input: Vec<i32> = vec![
            "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
            "127", "219", "299", "277", "309", "576",
        ]
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

        let value = slide_until_sum_of_any_2_in_last_25_is_not_current_value(&input);

        assert_eq!(5, value);
    }
}
