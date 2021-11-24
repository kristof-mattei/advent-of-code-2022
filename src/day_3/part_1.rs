use crate::{shared::AoCResult, utils::read_file};

fn descent_and_go_right(lines: &[String], row: usize, col: usize, mut trees: u32) -> u32 {
    match lines.get(row) {
        Some(line) => {
            if line.chars().nth(col) == Some('#') {
                trees += 1;
            }

            descent_and_go_right(lines, row + 1, (col + 3) % line.len(), trees)
        }
        None => trees,
    }
}

// https://adventofcode.com/2020/day/3
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_3/input.txt".into())?;

    let result = descent_and_go_right(&split, 0, 0, 0);

    Ok(AoCResult::Ofu32(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(AoCResult::Ofu32(191), find_solution().unwrap());
    }
}
