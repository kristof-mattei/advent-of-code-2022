use std::cmp;

use regex::Regex;

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

fn parse_lines(lines: &[String]) -> Vec<VentLine> {
    let regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    let mut vent_lines = Vec::new();

    for line in lines {
        let captures = regex.captures(line.trim()).unwrap();

        let x1 = captures[1].parse::<u32>().unwrap();
        let y1 = captures[2].parse::<u32>().unwrap();
        let x2 = captures[3].parse::<u32>().unwrap();
        let y2 = captures[4].parse::<u32>().unwrap();

        let vent_line = VentLine::new(x1, y1, x2, y2);

        if vent_line.is_horizontal_or_vertical() {
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

    let count_of_2: usize = (&field)
        .iter()
        .map(|r| r.iter().filter(|c| **c >= 2).count())
        .sum();

    count_of_2 as u32
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

    for x in &x_range {
        for y in &y_range {
            in_between.push((*x, *y));
        }
    }

    in_between
}

pub fn find_solution() -> u32 {
    let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

    let vent_lines: Vec<VentLine> = parse_lines(&lines);

    let overlap_of_2 = calculate_overlap_of_2(&vent_lines);

    overlap_of_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(find_solution(), 4993);
    }

    #[test]
    fn playground() {
        let lines: Vec<String> = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        let vent_lines: Vec<VentLine> = parse_lines(&lines);

        let overlap_of_2 = calculate_overlap_of_2(&vent_lines);

        assert_eq!(overlap_of_2, 5);
    }
}
