use advent_of_code_2022::shared::{PartSolution, Parts};

advent_of_code_2022::solution!("VJSFHWGFT", "LCTQFBVZV");

#[derive(Debug)]
struct Block(char);

struct Field {
    stacks: Vec<Vec<Block>>,
}

struct Operation((usize, usize, usize));

fn parse_lines(input: &str) -> (Field, Vec<Operation>) {
    let lines = input.lines().collect::<Vec<_>>();

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

            let _: Option<&str> = iter.next();
            let no_of_blocks = iter.next().map(|v| v.parse::<usize>().unwrap()).unwrap();
            let _: Option<&str> = iter.next();
            let from = iter.next().map(|v| v.parse::<usize>().unwrap()).unwrap();
            let _: Option<&str> = iter.next();
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
        .filter_map(|stack| stack.last().map(|&Block(c)| c))
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

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let (field, operations) = parse_lines(input);

        let top = execute_orders_9000(field, operations);

        top.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let (field, operations) = parse_lines(input);

        let top = execute_orders_9001(field, operations);

        top.into()
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
                PartSolution::String("VJSFHWGFT".into()),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::String("CMZ".into()),
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
                PartSolution::String("LCTQFBVZV".into()),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::String("MCD".into()),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
