use crate::shared::{Day, PartSolution};

fn determine_direction(directions: &[Direction]) -> (u32, u32) {
    let mut forward = 0;
    let mut depth = 0;

    for direction in directions {
        match direction {
            Direction::Up(x) => depth -= x,
            Direction::Down(x) => depth += x,
            Direction::Forward(x) => forward += x,
        }
    }
    (forward, depth)
}

fn determine_direction_2(directions: &[Direction]) -> (u32, u32) {
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;

    for direction in directions {
        match direction {
            Direction::Up(x) => aim -= x,
            Direction::Down(x) => aim += x,
            Direction::Forward(x) => {
                forward += x;
                depth += aim * x;
            }
        }
    }
    (forward, depth)
}

enum Direction {
    Up(u32),
    Down(u32),
    Forward(u32),
}

fn parse_lines(lines: &[String]) -> Vec<Direction> {
    let mut instructions = Vec::new();
    for line in lines {
        let pieces: Vec<&str> = line.split(' ').collect();

        let instruction = match (pieces[0], pieces[1].parse::<u32>().unwrap()) {
            ("forward", x) => Direction::Forward(x),
            ("down", x) => Direction::Down(x),
            ("up", x) => Direction::Up(x),
            _ => panic!("OMG what did you give me?"),
        };

        instructions.push(instruction);
    }

    instructions
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let instructions = parse_lines(&lines);

        let (horizontal_position, depth) = determine_direction(&instructions);

        PartSolution::U32(horizontal_position * depth)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();
        let instructions = parse_lines(&lines);

        let (horizontal_position, depth) = determine_direction_2(&instructions);

        PartSolution::U32(horizontal_position * depth)
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
            day_2::{determine_direction, parse_lines, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(2_070_300));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let instructions = parse_lines(&lines);

            let (horizontal_position, depth) = determine_direction(&instructions);

            assert_eq!(horizontal_position, 15);
            assert_eq!(depth, 10);
        }
    }
    mod part_2 {
        use crate::{
            day_2::{determine_direction_2, parse_lines, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U32(2_078_985_210));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let instructions = parse_lines(&lines);

            let (horizontal_position, depth) = determine_direction_2(&instructions);

            assert_eq!(horizontal_position, 15);
            assert_eq!(depth, 60);
        }
    }
}
