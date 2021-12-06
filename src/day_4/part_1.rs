use std::cell::Cell;

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
}

impl Playfield {
    fn new(playfield: [[PlayfieldCell; PLAYFIELD_SIZE]; PLAYFIELD_SIZE]) -> Playfield {
        Playfield { inner: playfield }
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

    (0, 0)
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

pub fn find_solution() -> u32 {
    let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

    let (drawings, play_fields) = parse_lines(&lines);

    let (gamma, epsilon) = calculate_winnings(&drawings, &play_fields);

    gamma * epsilon
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(find_solution(), 23177);
    }

    #[test]
    fn parses_drawings() {
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

        let (drawings, _) = parse_lines(&lines);

        assert_eq!(
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ],
            drawings
        );
    }

    #[test]
    fn parses_fields() {
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

        let (unmarked_sum, drawn) = calculate_winnings(&drawings, &play_fields);

        assert_eq!(unmarked_sum, 188);
        assert_eq!(drawn, 24);
    }
}
