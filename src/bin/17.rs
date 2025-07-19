use std::sync::LazyLock;

use advent_of_code_2022::shared::{PartSolution, Parts};
use hashbrown::HashMap;

advent_of_code_2022::solution!(3130, 1_556_521_739_139_usize);

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Block,
    Settled,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Cell::Empty => '.',
            Cell::Block => '@',
            Cell::Settled => '#',
        };

        write!(f, "{}", c)
    }
}

static PIECES: LazyLock<Vec<Vec<Vec<Cell>>>> = LazyLock::new(|| {
    // these are stored bottom to top
    let line = vec![vec![Cell::Block; 4]];

    let cross = vec![
        vec![Cell::Empty, Cell::Block, Cell::Empty],
        vec![Cell::Block, Cell::Block, Cell::Block],
        vec![Cell::Empty, Cell::Block, Cell::Empty],
    ];

    let reverse_l = vec![
        vec![Cell::Block, Cell::Block, Cell::Block],
        vec![Cell::Empty, Cell::Empty, Cell::Block],
        vec![Cell::Empty, Cell::Empty, Cell::Block],
    ];

    let bar = vec![vec![Cell::Block]; 4];

    let block = vec![vec![Cell::Block; 2]; 2];

    vec![line, cross, reverse_l, bar, block]
});

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Direction::Right),
            '<' => Ok(Direction::Left),
            _ => Err("Invalid direction"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .trim()
        .chars()
        .map(|c| c.try_into().unwrap())
        .collect()
}

fn drop_blocks(input: &str, target: usize) -> PartSolution {
    let jets = parse_input(input);

    let mut field: Vec<[Cell; 7]> = vec![];

    let mut jet_count = 0;
    let mut piece_count = 0;

    let mut repeats_from_cache = 0_usize;

    let mut cache = HashMap::new();

    let mut top = 0;

    while piece_count != target {
        let piece = &PIECES[piece_count % PIECES.len()];

        // get the top row
        let mut block_row_index = top + 3;

        let mut block_column_index = 2;

        // until blocked
        loop {
            // left / right
            let direction = &jets[jet_count % jets.len()];

            jet_count += 1;

            (block_row_index, block_column_index) = get_new_block_position_direction(
                &field,
                block_row_index,
                block_column_index,
                piece,
                *direction,
            );

            // drop
            let Some((new_block_row_index, new_block_column_index)) =
                get_new_block_position_down(&field, block_row_index, block_column_index, piece)
            else {
                if field.len() < block_row_index + piece.len() {
                    field.resize(block_row_index + piece.len(), [Cell::Empty; 7]);
                }
                // mark the block's pieces as settled
                for row_index in 0..piece.len() {
                    for column_index in 0..piece[0].len() {
                        if matches!(piece[row_index][column_index], Cell::Block) {
                            field[block_row_index + row_index][block_column_index + column_index] =
                                Cell::Settled;

                            top = top.max(block_row_index + row_index + 1);
                        }
                    }
                }

                break;
            };

            (block_row_index, block_column_index) = (new_block_row_index, new_block_column_index);
        }

        if repeats_from_cache == 0 {
            let key = (piece_count % PIECES.len(), jet_count % jets.len());

            if let Some(&(2, ref old_piece_count, old_top)) = cache.get(&key) {
                let delta_top = top - old_top;
                let delta_piece_count = piece_count - old_piece_count;

                let repeats = (target - piece_count) / delta_piece_count;

                repeats_from_cache += repeats * delta_top;
                piece_count += repeats * delta_piece_count;
            }

            cache
                .entry(key)
                .and_modify(
                    |&mut (ref mut amount, ref mut old_piece_count, ref mut old_top)| {
                        *amount += 1;
                        *old_piece_count = piece_count;
                        *old_top = top;
                    },
                )
                .or_insert((1, piece_count, top));
        }

        piece_count += 1;
    }

    PartSolution::USize(top + repeats_from_cache)
}

fn get_new_block_position_direction(
    field: &[[Cell; 7]],
    block_row_index: usize,
    start_block_column_index: usize,
    block: &[Vec<Cell>],
    direction: Direction,
) -> (usize, usize) {
    let new_block_column_index = if matches!(direction, Direction::Left) {
        start_block_column_index.checked_sub(1)
    } else {
        (start_block_column_index + block[0].len() <= 6).then_some(start_block_column_index + 1)
    };

    let Some(new_block_column_index) = new_block_column_index else {
        return (block_row_index, start_block_column_index);
    };

    for row_index in 0..block.len() {
        for column_index in 0..block[0].len() {
            if matches!(block[row_index][column_index], Cell::Block)
                && matches!(
                    field
                        .get(block_row_index + row_index)
                        .and_then(|row| row.get(new_block_column_index + column_index)),
                    Some(&Cell::Settled)
                )
            {
                return (block_row_index, start_block_column_index);
            }
        }
    }

    (block_row_index, new_block_column_index)
}

fn get_new_block_position_down(
    field: &[[Cell; 7]],
    start_block_row_index: usize,
    block_column_index: usize,
    block: &[Vec<Cell>],
) -> Option<(usize, usize)> {
    let new_block_row_index = start_block_row_index.checked_sub(1)?;

    for row_index in 0..block.len() {
        for column_index in 0..block[0].len() {
            if matches!(block[row_index][column_index], Cell::Block)
                && matches!(
                    field
                        .get(new_block_row_index + row_index)
                        .and_then(|row| row.get(block_column_index + column_index)),
                    Some(&Cell::Settled)
                )
            {
                return None;
            }
        }
    }

    Some((new_block_row_index, block_column_index))
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        drop_blocks(input, 2022)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        drop_blocks(input, 1_000_000_000_000)
    }
}

#[cfg(test)]
mod tests {
    mod part_1 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts as _};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(3130),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(3068),
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts as _};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(1_556_521_739_139_usize),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(1_514_285_714_288),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
