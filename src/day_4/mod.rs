use std::cell::Cell;

use crate::shared::{Day, PartSolution};

const PLAYFIELD_SIZE: usize = 5;

#[derive(PartialEq, Debug)]
struct PlayfieldCell {
    number: u32,
    drawn: Cell<bool>,
}

impl PlayfieldCell {
    fn new(number: u32) -> PlayfieldCell {
        PlayfieldCell {
            number,
            drawn: Cell::new(false),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Playfield {
    inner: [[PlayfieldCell; PLAYFIELD_SIZE]; PLAYFIELD_SIZE],
    dont_consider_anymore: Cell<bool>,
}

impl Playfield {
    fn new(playfield: [[PlayfieldCell; PLAYFIELD_SIZE]; PLAYFIELD_SIZE]) -> Playfield {
        Playfield {
            inner: playfield,
            dont_consider_anymore: Cell::new(false),
        }
    }
}

impl Playfield {
    fn mark_number(&self, number: u32) {
        for row in &self.inner {
            for c in row {
                if c.number == number {
                    c.drawn.set(true);

                    // don't stop, we may have the same number in the same field
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        // for row in &self.inner {
        //     if row.iter().all(|f| f.drawn.get()) {
        //         return true;
        //     }
        // }

        // by row
        for row_index in 0..PLAYFIELD_SIZE {
            let mut drawn_count: u32 = 0;

            for column_index in 0..PLAYFIELD_SIZE {
                if self.inner[row_index][column_index].drawn.get() {
                    drawn_count += 1;
                }
            }

            if drawn_count == PLAYFIELD_SIZE as u32 {
                return true;
            }
        }

        // by column
        for column_index in 0..PLAYFIELD_SIZE {
            let mut drawn_count: u32 = 0;

            for row_index in 0..PLAYFIELD_SIZE {
                if self.inner[row_index][column_index].drawn.get() {
                    drawn_count += 1;
                }
            }

            if drawn_count == PLAYFIELD_SIZE as u32 {
                return true;
            }
        }

        false
    }

    fn unmarked_sum(&self) -> u32 {
        self.inner.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .fold(0, |acc, c| acc + (if c.drawn.get() { 0 } else { c.number }))
        })
    }
}

fn calculate_winnings(drawings: &[u32], playfields: &[Playfield]) -> (u32, u32) {
    for drawing in drawings {
        for playfield in playfields {
            playfield.mark_number(*drawing);

            if playfield.is_winner() {
                return (playfield.unmarked_sum(), *drawing);
            }
        }
    }

    panic!("No winner found")
}

fn calculate_last_winning_board_2(drawings: &[u32], playfields: &[Playfield]) -> (u32, u32) {
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

fn parse_lines(lines: &[String]) -> (Vec<u32>, Vec<Playfield>) {
    let drawings = lines[0]
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    // top line is our drawings
    let raw_playfields: Vec<_> = lines[1..lines.len()]
        .split(String::is_empty)
        .filter(|f| !f.is_empty())
        .collect();

    let mut playfields = Vec::<Playfield>::new();

    for raw_playfield in raw_playfields {
        let mut rows: Vec<[PlayfieldCell; PLAYFIELD_SIZE]> = Vec::new();

        for line in raw_playfield {
            let row_vec: Vec<PlayfieldCell> = line
                .split(' ')
                .filter(|f| !f.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .map(PlayfieldCell::new)
                .collect();

            let row: [PlayfieldCell; PLAYFIELD_SIZE] =
                row_vec.try_into().unwrap_or_else(|v: Vec<PlayfieldCell>| {
                    panic!(
                        "Tried to parse vector of wrong size, vector length: {}",
                        v.len()
                    )
                });

            rows.push(row);
        }

        let playfield = Playfield::new(rows.try_into().unwrap_or_else(
            |v: Vec<[PlayfieldCell; PLAYFIELD_SIZE]>| {
                panic!(
                    "Tried to parse vector of wrong size, vector length: {}",
                    v.len()
                )
            },
        ));

        playfields.push(playfield);
    }

    (drawings, playfields)
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let (drawings, play_fields) = parse_lines(&lines);

        let (winning_board_sum_unmarked, drawing) = calculate_winnings(&drawings, &play_fields);

        PartSolution::U32(winning_board_sum_unmarked * drawing)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let (drawings, play_fields) = parse_lines(&lines);

        let (last_winning_board_sum_unmarked, last_drawing) =
            calculate_last_winning_board_2(&drawings, &play_fields);

        PartSolution::U32(last_winning_board_sum_unmarked * last_drawing)
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<String> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {
        use crate::{
            day_4::{
                calculate_winnings, parse_lines, test::get_example, Playfield, PlayfieldCell,
                Solution,
            },
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(23177));
        }

        #[test]
        fn parses_drawings() {
            let lines = get_example();

            let (drawings, _) = parse_lines(&lines);

            assert_eq!(
                vec![
                    7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18,
                    20, 8, 19, 3, 26, 1
                ],
                drawings
            );
        }

        #[test]
        fn example() {
            let lines = get_example();

            let (_, playfields) = parse_lines(&lines);

            let playfield_1 = Playfield::new([
                [
                    PlayfieldCell::new(22),
                    PlayfieldCell::new(13),
                    PlayfieldCell::new(17),
                    PlayfieldCell::new(11),
                    PlayfieldCell::new(0),
                ],
                [
                    PlayfieldCell::new(8),
                    PlayfieldCell::new(2),
                    PlayfieldCell::new(23),
                    PlayfieldCell::new(4),
                    PlayfieldCell::new(24),
                ],
                [
                    PlayfieldCell::new(21),
                    PlayfieldCell::new(9),
                    PlayfieldCell::new(14),
                    PlayfieldCell::new(16),
                    PlayfieldCell::new(7),
                ],
                [
                    PlayfieldCell::new(6),
                    PlayfieldCell::new(10),
                    PlayfieldCell::new(3),
                    PlayfieldCell::new(18),
                    PlayfieldCell::new(5),
                ],
                [
                    PlayfieldCell::new(1),
                    PlayfieldCell::new(12),
                    PlayfieldCell::new(20),
                    PlayfieldCell::new(15),
                    PlayfieldCell::new(19),
                ],
            ]);

            let playfield_2 = Playfield::new([
                [
                    PlayfieldCell::new(3),
                    PlayfieldCell::new(15),
                    PlayfieldCell::new(0),
                    PlayfieldCell::new(2),
                    PlayfieldCell::new(22),
                ],
                [
                    PlayfieldCell::new(9),
                    PlayfieldCell::new(18),
                    PlayfieldCell::new(13),
                    PlayfieldCell::new(17),
                    PlayfieldCell::new(5),
                ],
                [
                    PlayfieldCell::new(19),
                    PlayfieldCell::new(8),
                    PlayfieldCell::new(7),
                    PlayfieldCell::new(25),
                    PlayfieldCell::new(23),
                ],
                [
                    PlayfieldCell::new(20),
                    PlayfieldCell::new(11),
                    PlayfieldCell::new(10),
                    PlayfieldCell::new(24),
                    PlayfieldCell::new(4),
                ],
                [
                    PlayfieldCell::new(14),
                    PlayfieldCell::new(21),
                    PlayfieldCell::new(16),
                    PlayfieldCell::new(12),
                    PlayfieldCell::new(6),
                ],
            ]);

            let playfield_3 = Playfield::new([
                [
                    PlayfieldCell::new(14),
                    PlayfieldCell::new(21),
                    PlayfieldCell::new(17),
                    PlayfieldCell::new(24),
                    PlayfieldCell::new(4),
                ],
                [
                    PlayfieldCell::new(10),
                    PlayfieldCell::new(16),
                    PlayfieldCell::new(15),
                    PlayfieldCell::new(9),
                    PlayfieldCell::new(19),
                ],
                [
                    PlayfieldCell::new(18),
                    PlayfieldCell::new(8),
                    PlayfieldCell::new(23),
                    PlayfieldCell::new(26),
                    PlayfieldCell::new(20),
                ],
                [
                    PlayfieldCell::new(22),
                    PlayfieldCell::new(11),
                    PlayfieldCell::new(13),
                    PlayfieldCell::new(6),
                    PlayfieldCell::new(5),
                ],
                [
                    PlayfieldCell::new(2),
                    PlayfieldCell::new(0),
                    PlayfieldCell::new(12),
                    PlayfieldCell::new(3),
                    PlayfieldCell::new(7),
                ],
            ]);

            assert_eq!(playfield_1, playfields[0]);
            assert_eq!(playfield_2, playfields[1]);
            assert_eq!(playfield_3, playfields[2]);
        }

        #[test]
        fn example_3() {
            let lines = get_example();

            let (drawings, play_fields) = parse_lines(&lines);

            let (unmarked_sum, drawn) = calculate_winnings(&drawings, &play_fields);

            assert_eq!(unmarked_sum, 188);
            assert_eq!(drawn, 24);
        }
    }

    mod part_2 {
        use crate::{
            day_4::{calculate_last_winning_board_2, parse_lines, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U32(6804));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let (drawings, play_fields) = parse_lines(&lines);

            let (unmarked_sum, drawn) = calculate_last_winning_board_2(&drawings, &play_fields);

            assert_eq!(unmarked_sum, 148);
            assert_eq!(drawn, 13);
        }
    }
}
