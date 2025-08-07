use advent_of_code_2022::shared::{PartSolution, Parts};

advent_of_code_2022::solution!(
    14040,
    PartSolution::Vec(vec![
        "####..##...##....##.####...##.####.#....".into(),
        "...#.#..#.#..#....#....#....#.#....#....".into(),
        "..#..#....#.......#...#.....#.###..#....".into(),
        ".#...#.##.#.......#..#......#.#....#....".into(),
        "#....#..#.#..#.#..#.#....#..#.#....#....".into(),
        "####..###..##...##..####..##..#....####.".into(),
    ])
);

#[derive(PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_lines(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        if &line[0..=3] == "addx" {
            instructions.push(Instruction::Addx(line[5..].parse::<i32>().unwrap()));
        } else {
            instructions.push(Instruction::Noop);
        }
    }
    instructions
}

fn draw_crt(instructions: Vec<Instruction>) -> Vec<Vec<char>> {
    // register
    let mut x: i32 = 1;

    let mut cycle: usize = 1;

    let mut instructions_iter = instructions.into_iter();

    let mut slot = Option::<Instruction>::None;

    let mut result = vec![vec![]];

    let mut slot_is_fresh;

    loop {
        slot_is_fresh = false;

        if let Some(Instruction::Noop) | None = slot {
            if let Some(next) = instructions_iter.next() {
                slot = Some(next);
                slot_is_fresh = true;
            } else {
                break;
            }
        }

        if cycle > 40 {
            cycle = 1;
            result.push(vec![]);
        }

        {
            let cycle = i32::try_from(cycle).unwrap();

            let last = result.last_mut().unwrap();

            if (cycle - 1) >= x - 1 && (cycle - 1) <= x + 1 {
                last.push('#');
            } else {
                last.push('.');
            }
        }

        if !slot_is_fresh && let Some(Instruction::Addx(addx)) = slot {
            x += addx;
            slot = None;
        }

        cycle += 1;
    }

    result
}

fn sum_signal_strength(instructions: Vec<Instruction>) -> i32 {
    // register
    let mut x: i32 = 1;
    let mut cycle: usize = 1;

    let mut instructions_iter = instructions.into_iter();

    let mut slot = Option::<Instruction>::None;

    // sum of signal strengths
    let mut sum = 0;

    let mut slot_is_fresh;

    loop {
        slot_is_fresh = false;

        if let Some(Instruction::Noop) | None = slot {
            if let Some(next) = instructions_iter.next() {
                slot = Some(next);
                slot_is_fresh = true;
            } else {
                break;
            }
        }

        if cycle >= 20 && (cycle - 20) % 40 == 0 {
            sum += i32::try_from(cycle).unwrap() * x;
        }

        if !slot_is_fresh && let Some(Instruction::Addx(addx)) = slot {
            x += addx;
            slot = None;
        }

        cycle += 1;
    }

    sum
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let sum = sum_signal_strength(parsed);

        sum.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let result = draw_crt(parsed)
            .into_iter()
            .map(|x| x.into_iter().collect::<String>())
            .collect::<Vec<String>>();

        PartSolution::Vec(result)
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
                PartSolution::I32(14040),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::I32(13140),
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
                PartSolution::Vec(vec![
                    "####..##...##....##.####...##.####.#....".into(),
                    "...#.#..#.#..#....#....#....#.#....#....".into(),
                    "..#..#....#.......#...#.....#.###..#....".into(),
                    ".#...#.##.#.......#..#......#.#....#....".into(),
                    "#....#..#.#..#.#..#.#....#..#.#....#....".into(),
                    "####..###..##...##..####..##..#....####.".into(),
                ]),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            let expected = vec![
                "##..##..##..##..##..##..##..##..##..##..",
                "###...###...###...###...###...###...###.",
                "####....####....####....####....####....",
                "#####.....#####.....#####.....#####.....",
                "######......######......######......####",
                "#######.......#######.......#######.....",
            ]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();

            assert_eq!(
                PartSolution::Vec(expected),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
