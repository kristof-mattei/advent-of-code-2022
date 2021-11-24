use crate::{day_7::part_1::parse_bags, utils::read_file};
use std::rc::Rc;

use super::part_1::Bag;

fn count_bags_recursive(bag: &Rc<Bag>) -> u32 {
    let children = bag.children.borrow();

    println!("Bag {}", &bag.name);

    children
        .iter()
        .map(|(c, b)| {
            let sum_of_children = count_bags_recursive(b);
            println!("Child {}*{} has {} children", c, b.name, sum_of_children);

            c + c * sum_of_children
        })
        .sum()
}

// https://adventofcode.com/2020/day/7
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    const BAG_NAME: &str = "shiny gold";
    let split = read_file("./src/day_7/input.txt".into())?;

    let bags = parse_bags(&split);

    Ok(count_bags_recursive(bags.get(BAG_NAME).unwrap()))
}

#[cfg(test)]
mod tests {
    use crate::day_7::part_1::parse_bag_line;

    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(172_246, find_solution().unwrap());
    }

    #[test]
    fn parse_bag_line_1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";

        let result = parse_bag_line(input);

        assert_eq!(
            result,
            (
                "light red".to_string(),
                vec![
                    (1, "bright white".to_string()),
                    (2, "muted yellow".to_string())
                ]
            ),
        );
    }

    #[test]
    fn parse_bag_line_2() {
        let input = "dark orange bags contain 3 bright white bags, 4 muted yellow bags.";

        let result = parse_bag_line(input);

        assert_eq!(
            result,
            (
                "dark orange".to_string(),
                vec![
                    (3, "bright white".to_string()),
                    (4, "muted yellow".to_string())
                ]
            ),
        );
    }

    #[test]
    fn parse_bag_line_3() {
        let input = "bright white bags contain 1 shiny gold bag.";

        let result = parse_bag_line(input);

        assert_eq!(
            result,
            (
                "bright white".to_string(),
                vec![(1, "shiny gold".to_string())]
            ),
        );
    }

    #[test]
    fn parse_bag_line_4() {
        let input = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.";

        let result = parse_bag_line(input);

        assert_eq!(
            result,
            (
                "muted yellow".to_string(),
                vec![(2, "shiny gold".to_string()), (9, "faded blue".to_string())]
            ),
        );
    }

    #[test]
    fn parse_bag_line_5() {
        let input = "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.";

        let result = parse_bag_line(input);

        assert_eq!(
            result,
            (
                "shiny gold".to_string(),
                vec![
                    (1, "dark olive".to_string()),
                    (2, "vibrant plum".to_string())
                ]
            ),
        );
    }

    #[test]
    fn parse_bag_line_6() {
        let input = "dark olive bags contain 3 faded blue bags, 4 dotted black bags.";

        let result = parse_bag_line(input);

        assert_eq!(
            result,
            (
                "dark olive".to_string(),
                vec![
                    (3, "faded blue".to_string()),
                    (4, "dotted black".to_string())
                ]
            ),
        );
    }

    #[test]
    fn parse_bag_line_7() {
        let input = "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.";

        let result = parse_bag_line(input);

        assert_eq!(
            result,
            (
                "vibrant plum".to_string(),
                vec![
                    (5, "faded blue".to_string()),
                    (6, "dotted black".to_string())
                ]
            ),
        );
    }

    #[test]
    fn parse_bag_line_8() {
        let input = "faded blue bags contain no other bags.";

        let result = parse_bag_line(input);

        assert_eq!(result, ("faded blue".to_string(), vec![]));
    }

    #[test]
    fn parse_bag_line_9() {
        let input = "dotted black bags contain no other bags.";

        let result = parse_bag_line(input);

        assert_eq!(result, ("dotted black".to_string(), vec![]));
    }

    #[test]
    fn big_test() {
        const BAG_NAME: &str = "shiny gold";

        let input = [
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];

        let lines: Vec<String> = input.map(|s| s.into()).into();

        let bags = parse_bags(&lines);

        let rst = count_bags_recursive(bags.get(BAG_NAME).unwrap());

        assert_eq!(rst, 126);
    }
}
