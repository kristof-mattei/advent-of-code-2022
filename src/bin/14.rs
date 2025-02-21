use std::collections::BTreeMap;
use std::collections::btree_map::Entry::{Occupied, Vacant};
use std::io::{Write, stdout};

use advent_of_code_2022::shared::{PartSolution, Parts};

advent_of_code_2022::solution!(5185, 23_751);

struct Step {
    row_index: usize,
    column_index: usize,
}

struct FieldInstructions {
    line_instructions: Vec<Vec<Step>>,
    max_row_index: usize,
    min_column_index: usize,
    max_column_index: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Contents {
    Sand,
    Rock,
    Start,
}

const SAND_START_COLUMN: usize = 500;

fn parse_lines(input: &str) -> FieldInstructions {
    let mut line_instructions = Vec::new();

    let mut min_column_index = usize::MAX;
    let mut max_column_index = 0usize;

    let mut max_row_index = 0usize;

    for line in input.lines() {
        let steps = line.split(" -> ");

        let mut instructions = Vec::new();

        for step in steps {
            let pieces = step
                .split(',')
                .map(|p| p.parse().unwrap())
                .collect::<Vec<_>>();

            let row_index = pieces[1];
            let column_index = pieces[0];

            min_column_index = usize::min(min_column_index, column_index);
            max_column_index = usize::max(max_column_index, column_index);

            max_row_index = usize::max(max_row_index, row_index);

            instructions.push(Step {
                row_index,
                column_index,
            });
        }

        line_instructions.push(instructions);
    }

    FieldInstructions {
        line_instructions,
        max_row_index,
        min_column_index,
        max_column_index,
    }
}

fn set_row_column_contents(
    field: &mut BTreeMap<usize, BTreeMap<usize, Contents>>,
    row_index: usize,
    column_index: usize,
    contents: Contents,
) {
    match field.entry(row_index) {
        Occupied(mut o) => {
            o.get_mut().insert(column_index, contents);
        },
        Vacant(v) => {
            v.insert(BTreeMap::from_iter([(column_index, contents)]));
        },
    }
}

fn get_row_column_contents(
    field: &BTreeMap<usize, BTreeMap<usize, Contents>>,
    row_index: usize,
    column_index: usize,
) -> Option<Contents> {
    field
        .get(&row_index)
        .and_then(|row| row.get(&column_index))
        .copied()
}

fn draw_line(field: &mut BTreeMap<usize, BTreeMap<usize, Contents>>, instructions: &[Step]) {
    for step in instructions.windows(2) {
        let start = &step[0];
        let end = &step[1];

        if start.column_index == end.column_index {
            // we move over x
            let start_row = start.row_index.min(end.row_index);
            let end_row = start.row_index.max(end.row_index);

            #[allow(clippy::needless_range_loop)]
            for row in start_row..=end_row {
                set_row_column_contents(field, row, start.column_index, Contents::Rock);
            }
        } else if start.row_index == end.row_index {
            // we move over y

            let start_column_index = start.column_index.min(end.column_index);
            let end_column_index = start.column_index.max(end.column_index);

            for column_index in start_column_index..=end_column_index {
                set_row_column_contents(field, start.row_index, column_index, Contents::Rock);
            }
        } else {
            panic!("Cannot go diagonal")
        }
    }
}

fn parse_field_instructions(field_instructions: FieldInstructions) -> usize {
    let mut field = BTreeMap::<usize, BTreeMap<usize, Contents>>::new();

    for instructions in field_instructions.line_instructions {
        draw_line(&mut field, &instructions);
    }

    set_row_column_contents(&mut field, 0, SAND_START_COLUMN, Contents::Start);

    let mut sands = 0;

    'new_sand: loop {
        let (mut sand_row_index, mut sand_column_index) = (0, SAND_START_COLUMN);

        'inner: loop {
            if sand_row_index + 1 == field_instructions.max_row_index + 1 {
                // into the abyss we go
                break 'new_sand;
            }

            let contents = get_row_column_contents(&field, sand_row_index + 1, sand_column_index);

            match contents {
                Some(Contents::Start) => {
                    panic!()
                },
                None => {
                    sand_row_index += 1;
                    // fall straight down
                    continue 'inner;
                },
                Some(Contents::Rock | Contents::Sand) => {
                    // can we fall left?
                    if sand_column_index == field_instructions.min_column_index {
                        // we can, and fall into the abyss
                        break 'new_sand;
                    }

                    // if left is empty, we fall left

                    if get_row_column_contents(&field, sand_row_index + 1, sand_column_index - 1)
                        .is_none()
                    {
                        sand_row_index += 1;
                        sand_column_index -= 1;

                        // fall down, left
                        continue 'inner;
                    }

                    // left is not empty

                    // can we fall to the right?
                    if sand_column_index + 1 == field_instructions.max_column_index {
                        // we can, but fall into the abyss
                        break 'new_sand;
                    }

                    // if right is empty, fall right
                    if get_row_column_contents(&field, sand_row_index + 1, sand_column_index + 1)
                        .is_none()
                    {
                        sand_row_index += 1;
                        sand_column_index += 1;

                        // fall down, left
                        continue 'inner;
                    }

                    // can't fall left, nor right. So we're stable. Give me a new sand
                    set_row_column_contents(
                        &mut field,
                        sand_row_index,
                        sand_column_index,
                        Contents::Sand,
                    );

                    sands += 1;

                    continue 'new_sand;
                },
            }
        }
    }

    dump_field(
        &field,
        field_instructions.min_column_index,
        field_instructions.max_column_index,
        field_instructions.max_row_index,
    );

    sands
}

fn get_row_column_contents_with_base(
    field: &BTreeMap<usize, BTreeMap<usize, Contents>>,
    row_index: usize,
    column_index: usize,
    max_row_index: usize,
) -> Option<Contents> {
    if row_index == max_row_index + 2 {
        Some(Contents::Rock)
    } else {
        get_row_column_contents(field, row_index, column_index)
    }
}

fn parse_field_instructions_part_2(mut field_instructions: FieldInstructions) -> usize {
    let mut field = BTreeMap::<usize, BTreeMap<usize, Contents>>::new();

    for instruction in field_instructions.line_instructions {
        draw_line(&mut field, &instruction);
    }

    set_row_column_contents(&mut field, 0, SAND_START_COLUMN, Contents::Start);

    let mut sands = 0;

    'new_sand: loop {
        let (mut sand_row_index, mut sand_column_index) = (0, SAND_START_COLUMN);

        'inner: loop {
            // endless stone layers 2 blocks down from the lowest stone
            let contents = get_row_column_contents_with_base(
                &field,
                sand_row_index + 1,
                sand_column_index,
                field_instructions.max_row_index,
            );

            match contents {
                Some(Contents::Start) => {
                    panic!()
                },
                None => {
                    sand_row_index += 1;
                    // fall straight down
                    continue 'inner;
                },
                Some(Contents::Rock | Contents::Sand) => {
                    // if left is empty, we fall left
                    if get_row_column_contents_with_base(
                        &field,
                        sand_row_index + 1,
                        sand_column_index - 1,
                        field_instructions.max_row_index,
                    )
                    .is_none()
                    {
                        sand_row_index += 1;
                        sand_column_index -= 1;

                        if field_instructions.min_column_index > sand_column_index {
                            field_instructions.min_column_index = sand_column_index;
                        }

                        // fall down, left
                        continue 'inner;
                    }

                    // left is not empty

                    // if right is empty, fall right
                    if get_row_column_contents_with_base(
                        &field,
                        sand_row_index + 1,
                        sand_column_index + 1,
                        field_instructions.max_row_index,
                    )
                    .is_none()
                    {
                        sand_row_index += 1;
                        sand_column_index += 1;

                        if field_instructions.max_column_index < sand_column_index {
                            field_instructions.max_column_index = sand_column_index;
                        }

                        // fall down, left
                        continue 'inner;
                    }

                    // can't fall left, nor right. So we're stable. Give me a new sand
                    set_row_column_contents(
                        &mut field,
                        sand_row_index,
                        sand_column_index,
                        Contents::Sand,
                    );

                    sands += 1;

                    if sand_row_index == 0 && sand_column_index == SAND_START_COLUMN {
                        // we just put our sand where the sand starts
                        // so we plugged the hole!
                        // we're done
                        break 'new_sand;
                    }

                    continue 'new_sand;
                },
            }
        }
    }

    dump_field_with_base(
        &field,
        field_instructions.min_column_index,
        field_instructions.max_column_index,
        field_instructions.max_row_index,
    );

    sands
}

fn dump_field(
    field: &BTreeMap<usize, BTreeMap<usize, Contents>>,
    min_column_index: usize,
    max_column_index: usize,
    row_count: usize,
) {
    if cfg!(debug_assertions) {
        let mut lock = stdout().lock();

        for row_index in 0..=row_count {
            for column_index in min_column_index..=max_column_index {
                let character = match get_row_column_contents(field, row_index, column_index) {
                    None => '.',
                    Some(Contents::Start) => '+',
                    Some(Contents::Sand) => 'o',
                    Some(Contents::Rock) => '#',
                };

                let _unused = write!(lock, "{}", character);
            }

            let _unused = writeln!(lock);
        }
    }
}

fn dump_field_with_base(
    field: &BTreeMap<usize, BTreeMap<usize, Contents>>,
    min_column_index: usize,
    max_column_index: usize,
    row_count: usize,
) {
    if cfg!(debug_assertions) {
        dump_field(
            field,
            min_column_index - 2,
            max_column_index + 2,
            row_count + 1,
        );

        let mut lock = stdout().lock();

        for _ in (min_column_index - 2)..=(max_column_index + 2) {
            let _unused = write!(lock, "#");
        }

        let _unused = writeln!(lock);
    }
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let sands = parse_field_instructions(parsed);

        sands.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let sands = parse_field_instructions_part_2(parsed);

        sands.into()
    }
}

#[cfg(test)]
mod tests {

    mod part_1 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(979),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(24),
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(29_044),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(93),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
