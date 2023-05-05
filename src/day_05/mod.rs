use crate::shared::{Day, PartSolution};

#[derive(Debug)]
struct Block(char);

struct Field {
    stacks: Vec<Vec<Block>>,
}

struct Operation((usize, usize, usize));

fn parse_lines(lines: &[&str]) -> (Field, Vec<Operation>) {
    // find the empty line, walk back 1 line, which has has the stacks, and then build the stacks
    let separator = lines.iter().position(|line| line.is_empty()).unwrap();

    let mut stacks = lines[separator - 1]
        .split_whitespace()
        .map(|_| Vec::<Block>::new())
        .collect::<Vec<_>>();

    for i in (0..=(separator - 2)).rev() {
        let as_chars = lines[i].chars().collect::<Vec<char>>();

        for (stack_index, block) in as_chars.chunks(4).enumerate() {
            if block[0] == '[' {
                stacks[stack_index].push(Block(block[1]));
            }
        }
    }

    let ops = lines
        .iter()
        .skip(separator + 1)
        .filter(|s| !str::is_empty(s))
        .map(|line| {
            let mut iter = line.split_whitespace();

            let _: Option<_> = iter.next();
            let no_of_blocks = iter.next().map(|v| v.parse::<usize>().unwrap()).unwrap();
            let _: Option<_> = iter.next();
            let from = iter.next().map(|v| v.parse::<usize>().unwrap()).unwrap();
            let _: Option<_> = iter.next();
            let to = iter.next().map(|v| v.parse::<usize>().unwrap()).unwrap();

            // from and to are 1-based in the input
            // but our stacks are 0 based
            Operation((no_of_blocks, from - 1, to - 1))
        })
        .collect::<Vec<_>>();

    (Field { stacks }, ops)
}

fn get_top_boxes(field: &Field) -> String {
    field
        .stacks
        .iter()
        .filter_map(|stack| stack.last().map(|Block(c)| *c))
        .collect::<String>()
}

fn execute_orders_9000(mut field: Field, ops: Vec<Operation>) -> String {
    for Operation((no_of_blocks, from, to)) in ops {
        for _ in 0..no_of_blocks {
            let block = field.stacks[from].pop().unwrap();
            field.stacks[to].push(block);
        }
    }

    get_top_boxes(&field)
}

fn execute_orders_9001(mut field: Field, ops: Vec<Operation>) -> String {
    for Operation((no_of_blocks, from, to)) in ops {
        let at = field.stacks[from].len() - no_of_blocks;
        let mut temp_stack = field.stacks[from].split_off(at);

        field.stacks[to].append(&mut temp_stack);
    }

    get_top_boxes(&field)
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let (field, operations) = parse_lines(&lines);

        let top = execute_orders_9000(field, operations);

        top.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let (field, operations) = parse_lines(&lines);

        let top = execute_orders_9001(field, operations);

        top.into()
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
        use super::super::test::get_example;
        use super::super::{execute_orders_9000, parse_lines, Solution};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::String("VJSFHWGFT".into()),
                (Solution {}).part_1()
            );
        }

        #[test]
        fn example() {
            let lines: Vec<&str> = get_example();

            let (field, operations) = parse_lines(&lines);

            let top = execute_orders_9000(field, operations);

            assert_eq!(top, "CMZ".to_string());
        }
    }

    mod part_2 {
        use super::super::test::get_example;
        use super::super::{execute_orders_9001, parse_lines, Solution};
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::String("LCTQFBVZV".into()),
                (Solution {}).part_2()
            );
        }

        #[test]
        fn example() {
            let lines: Vec<&str> = get_example();

            let (field, operations) = parse_lines(&lines);

            let top = execute_orders_9001(field, operations);

            assert_eq!(top, "MCD".to_string());
        }
    }
}
