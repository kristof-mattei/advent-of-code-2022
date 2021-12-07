use crate::day_4::part_1::{parse_lines, Playfield};

fn calculate_last_winning_board(drawings: &[u32], playfields: &[Playfield]) -> (u32, u32) {
    let mut last_winning_board_unmarked: u32 = 0;
    let mut last_drawing: u32 = 0;

    for drawing in drawings {
        for playfield in playfields {
            if playfield.dont_consider_anymore.get() {
                continue;
            }

            playfield.mark_number(*drawing);

            if playfield.is_winner() {
                last_winning_board_unmarked = playfield.unmarked_sum();
                last_drawing = *drawing;
                playfield.dont_consider_anymore.set(true);
            }
        }
    }

    (last_winning_board_unmarked, last_drawing)
}

pub fn find_solution() -> u32 {
    let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

    let (drawings, play_fields) = parse_lines(&lines);

    let (gamma, epsilon) = calculate_last_winning_board(&drawings, &play_fields);

    gamma * epsilon
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(find_solution(), 6804);
    }

    #[test]
    fn playground() {
        let lines: Vec<String> = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        let (drawings, play_fields) = parse_lines(&lines);

        let (unmarked_sum, drawn) = calculate_last_winning_board(&drawings, &play_fields);

        assert_eq!(unmarked_sum, 148);
        assert_eq!(drawn, 13);
    }
}
