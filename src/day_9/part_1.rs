use crate::{shared::AoCResult, utils::read_file};

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
}
