use std::cmp;

use regex::Regex;

use crate::shared::{Day, PartSolution};

#[derive(Debug)]
struct VentLine {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl VentLine {
    fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> VentLine {
        VentLine { x1, y1, x2, y2 }
    }
    fn is_horizontal_or_vertical(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }
}

fn parse_lines(lines: &[String], include_diagonal: bool) -> Vec<VentLine> {
    let regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    let mut vent_lines = Vec::new();

    for line in lines {
        let captures = regex.captures(line.trim()).unwrap();

        let x1 = captures[1].parse::<u32>().unwrap();
        let y1 = captures[2].parse::<u32>().unwrap();
        let x2 = captures[3].parse::<u32>().unwrap();
        let y2 = captures[4].parse::<u32>().unwrap();

        let vent_line = VentLine::new(x1, y1, x2, y2);

        if vent_line.is_horizontal_or_vertical() || include_diagonal {
            vent_lines.push(vent_line);
        }
    }

    vent_lines
}

fn calculate_overlap_of_2(vent_lines: &[VentLine]) -> u32 {
    let x_max = vent_lines
        .iter()
        .map(|vl| cmp::max(vl.x1, vl.x2))
        .max()
        .unwrap();
    let y_max = vent_lines
        .iter()
        .map(|vl| cmp::max(vl.y1, vl.y2))
        .max()
        .unwrap();

    // println!("x max: {}, y max: {}", x_max, y_max);

    let mut field: Vec<Vec<u32>> = Vec::new();

    for _ in 0..=y_max {
        field.push(
            std::iter::repeat(0)
                .take((x_max + 1) as usize)
                .collect::<Vec<_>>(),
        );
    }

    for vent_line in vent_lines {
        for (x, y) in &generate_coordinates_in_between(vent_line) {
            let val = field[((*y) as usize)][(*x) as usize];

            field[((*y) as usize)][(*x) as usize] = val + 1;
        }
    }

    // for row in &field {
    //     for c in row {
    //         match c {
    //             0 => print!("."),
    //             x => print!("{}", x),
    //         }
    //     }

    //     println!();
    // }

    let count_of_over_2: usize = (&field)
        .iter()
        .map(|r| r.iter().filter(|c| **c >= 2).count())
        .sum();

    count_of_over_2 as u32
}

fn generate_coordinates_in_between(vent_line: &VentLine) -> Vec<(u32, u32)> {
    let mut in_between: Vec<(u32, u32)> = Vec::new();

    // let x_diff = cmp::max(vent_line.x1, vent_line.x2) - cmp::min(vent_line.x1, vent_line.x2);
    // let y_diff = cmp::max(vent_line.y1, vent_line.y2) - cmp::min(vent_line.y1, vent_line.y2);

    // println!("Vec: {:?}", vent_line);

    let x_range: Vec<u32> = if vent_line.x1 <= vent_line.x2 {
        (vent_line.x1..=vent_line.x2).collect()
    } else {
        (vent_line.x2..=vent_line.x1).rev().collect()
    };

    // println!("x_range: {:?}", x_range);

    let y_range: Vec<u32> = if vent_line.y1 <= vent_line.y2 {
        (vent_line.y1..=vent_line.y2).collect()
    } else {
        (vent_line.y2..=vent_line.y1).rev().collect()
    };

    // println!("y_range: {:?}", y_range);

    // diagonal
    if x_range.len() == y_range.len() {
        for i in 0..x_range.len() {
            in_between.push((x_range[i], y_range[i]));
        }
    } else {
        for x in &x_range {
            for y in &y_range {
                in_between.push((*x, *y));
            }
        }
    }

    in_between
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let vent_lines: Vec<VentLine> = parse_lines(&lines, false);

        let overlap_of_2 = calculate_overlap_of_2(&vent_lines);

        PartSolution::U32(overlap_of_2)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let vent_lines: Vec<VentLine> = parse_lines(&lines, true);

        let overlap_of_2 = calculate_overlap_of_2(&vent_lines);

        PartSolution::U32(overlap_of_2)
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
            day_5::{calculate_overlap_of_2, parse_lines, test::get_example, Solution, VentLine},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(4993));
        }

        #[test]
        fn example() {
            let lines: Vec<String> = get_example();

            let vent_lines: Vec<VentLine> = parse_lines(&lines, false);

            let overlap_of_2 = calculate_overlap_of_2(&vent_lines);

            assert_eq!(overlap_of_2, 5);
        }
    }

    mod part_2 {
        use crate::{
            day_5::{calculate_overlap_of_2, parse_lines, test::get_example, Solution, VentLine},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U32(21101));
        }

        #[test]
        fn example() {
            let lines: Vec<String> = get_example();

            let vent_lines: Vec<VentLine> = parse_lines(&lines, true);

            let overlap_of_2 = calculate_overlap_of_2(&vent_lines);

            assert_eq!(overlap_of_2, 12);
        }
    }
}
