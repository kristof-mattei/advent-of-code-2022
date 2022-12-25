use crate::shared::{Day, PartSolution};

#[derive(PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_lines(lines: &[&str]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in lines {
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

        if !slot_is_fresh {
            if let Some(Instruction::Addx(addx)) = slot {
                x += addx;
                slot = None;
            }
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

        if !slot_is_fresh {
            if let Some(Instruction::Addx(addx)) = slot {
                x += addx;
                slot = None;
            }
        }

        cycle += 1;
    }

    sum
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let parsed = parse_lines(&lines);

        let sum = sum_signal_strength(parsed);

        sum.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let parsed = parse_lines(&lines);

        let result = draw_crt(parsed)
            .into_iter()
            .map(|x| x.into_iter().collect::<String>())
            .collect::<Vec<String>>();

        PartSolution::Vec(result)
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {
        use super::super::{parse_lines, sum_signal_strength, Solution};
        use super::get_example;
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::I32(14040), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            let sum = sum_signal_strength(parsed);

            assert_eq!(13140, sum);
        }
    }

    mod part_2 {
        use super::super::{draw_crt, parse_lines, Solution};
        use super::get_example;
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            let expected = vec![
                "####..##...##....##.####...##.####.#....".into(),
                "...#.#..#.#..#....#....#....#.#....#....".into(),
                "..#..#....#.......#...#.....#.###..#....".into(),
                ".#...#.##.#.......#..#......#.#....#....".into(),
                "#....#..#.#..#.#..#.#....#..#.#....#....".into(),
                "####..###..##...##..####..##..#....####.".into(),
            ];

            assert_eq!(PartSolution::Vec(expected), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let parsed = parse_lines(&lines);

            let result = draw_crt(parsed);

            let expected = vec![
                "##..##..##..##..##..##..##..##..##..##..",
                "###...###...###...###...###...###...###.",
                "####....####....####....####....####....",
                "#####.....#####.....#####.....#####.....",
                "######......######......######......####",
                "#######.......#######.......#######.....",
            ]
            .iter()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

            for i in 0..expected.len() {
                assert_eq!(expected[i], result[i]);
            }
        }
    }
}
