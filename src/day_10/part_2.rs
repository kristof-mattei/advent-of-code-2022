use crate::{
    shared::{AoCError, AoCResult},
    utils::read_file,
};

// https://adventofcode.com/2020/day/9
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_9/input.txt".into())?;
    let _input: Vec<u32> = split.iter().map(|s| s.parse::<u32>().unwrap()).collect();

    Ok(AoCResult::Ofu32(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(AoCResult::Ofu32(0), find_solution().unwrap());
    }

    #[test]
    fn test_example() {
        let input: Vec<u32> = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"]
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        // ...
        // assert_eq!(127, value);
    }
}
