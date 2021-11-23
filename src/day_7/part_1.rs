use crate::utils::read_file;

// https://adventofcode.com/2020/day/7
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let _split = read_file("./src/day_7/input.txt".into())?;

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(0, find_solution().unwrap());
    }
}
