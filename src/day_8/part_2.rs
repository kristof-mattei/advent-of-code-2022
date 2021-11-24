use crate::{shared::AoCResult, utils::read_file};

// https://adventofcode.com/2020/day/8
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    let _split = read_file("./src/day_8/input.txt".into())?;

    Ok(AoCResult::Ofu32(0))
}

#[cfg(test)]
mod tests {}
