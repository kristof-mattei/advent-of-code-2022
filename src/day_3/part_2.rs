use crate::{shared::AoCResult, utils::read_file};

fn descent_and_go_right(
    lines: &[String],
    row: usize,
    col: usize,
    mut trees: u32,
    down: usize,
    right: usize,
) -> u32 {
    match &lines.get(row) {
        Some(line) => {
            if line.chars().nth(col) == Some('#') {
                trees += 1;
            }

            descent_and_go_right(
                lines,
                row + down,
                (col + right) % line.len(),
                trees,
                down,
                right,
            )
        }
        None => trees,
    }
}

// https://adventofcode.com/2020/day/3
pub fn find_solution() -> Result<AoCResult, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_3/input.txt".into())?;

    let result1 = descent_and_go_right(&split, 0, 0, 0, 1, 1);
    let result2 = descent_and_go_right(&split, 0, 0, 0, 1, 3);
    let result3 = descent_and_go_right(&split, 0, 0, 0, 1, 5);
    let result4 = descent_and_go_right(&split, 0, 0, 0, 1, 7);
    let result5 = descent_and_go_right(&split, 0, 0, 0, 2, 1);

    Ok(AoCResult::Ofu32(
        result1 * result2 * result3 * result4 * result5,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(AoCResult::Ofu32(1_478_615_040), find_solution().unwrap());
    }
}
