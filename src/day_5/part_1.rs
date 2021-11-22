use crate::utils::read_file;

fn parse_seat(seat_line: &str) -> (u32, u32) {
    const LOWER_BITS_ROW: u32 = 0;
    const UPPER_BITS_ROW: u32 = LOWER_BITS_ROW + 6;
    const LOWER_BITS_COL: u32 = UPPER_BITS_ROW + 1;
    const UPPER_BITS_COL: u32 = LOWER_BITS_COL + 2;

    let mut pieces = seat_line.chars();

    let mut row: u32 = 0;
    let mut column: u32 = 0;

    for i in 0..=UPPER_BITS_ROW {
        match pieces.next().unwrap() {
            'F' => (),
            'B' => row |= 0b1 << (UPPER_BITS_ROW - i),
            _ => panic!("Not F or B"),
        }
    }

    for i in 7..=UPPER_BITS_COL {
        match pieces.next().unwrap() {
            'L' => (),
            'R' => column |= 0b1 << (UPPER_BITS_COL - i),
            _ => panic!("Not L or R"),
        }
    }

    (row, column)
}

// https://adventofcode.com/2020/day/5
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_5/input.txt".into())?;

    let max = split
        .iter()
        .map(|l| parse_seat(l))
        .map(|(r, c)| r * 8 + c)
        .max()
        .unwrap();

    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(951, find_solution().unwrap());
    }

    #[test]
    fn seat_id_1() {
        let seat = "FBFBBFFRLR";

        assert_eq!(parse_seat(seat), (44, 5));
    }

    #[test]
    fn seat_id_2() {
        let seat = "BFFFBBFRRR";

        assert_eq!(parse_seat(seat), (70, 7));
    }

    #[test]
    fn seat_id_3() {
        let seat = "FFFBBBFRRR";

        assert_eq!(parse_seat(seat), (14, 7));
    }
    #[test]
    fn seat_id_4() {
        let seat = "BBFFBBFRLL";

        assert_eq!(parse_seat(seat), (102, 4));
    }
}
