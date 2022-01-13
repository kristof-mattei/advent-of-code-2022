use std::fmt::Debug;

use crate::shared::{Day, PartSolution};

static COMMA_WIDTH: usize = 1;
static BRACE_WIDTH: usize = 1;

#[derive(PartialEq, Eq, Clone)]
enum Snailfish {
    Value(u32),
    Pair((Box<Snailfish>, Box<Snailfish>)),
}

impl Snailfish {
    fn extract_value(self) -> Option<u32> {
        match self {
            Snailfish::Value(x) => Some(x),
            Snailfish::Pair(_) => None,
        }
    }

    fn fmt_with_depth(&self, depth: u32) -> String {
        match self {
            Snailfish::Value(x) => format!("\x1b[{}m{}\x1b[0m", depth + 30, x),
            Snailfish::Pair((l, r)) => format!(
                "\x1b[{}m[{}\x1b[{}m,{}\x1b[{}m]\x1b[0m",
                depth + 30,
                l.fmt_with_depth(depth + 1),
                depth + 30,
                r.fmt_with_depth(depth + 1),
                depth + 30,
            ),
        }
    }
}

impl Debug for Snailfish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_depth(0))
    }
}

enum Side {
    Left,
    Right,
}

struct ExplodedSnailfish {
    snailfish: Snailfish,
    push_to_left: Option<u32>,
    push_to_right: Option<u32>,
    exploded: bool,
}

impl ExplodedSnailfish {
    fn new(
        snailfish: Snailfish,
        push_to_left: Option<u32>,
        push_to_right: Option<u32>,
        exploded: bool,
    ) -> Self {
        Self {
            snailfish,
            push_to_left,
            push_to_right,
            exploded,
        }
    }
}

struct SplitSnailfish {
    snailfish: Snailfish,
    split: bool,
}

impl SplitSnailfish {
    fn new(snailfish: Snailfish, split: bool) -> Self {
        Self { snailfish, split }
    }
}

fn parse_snailfish_pair(chars: &[char], offset: usize) -> (Snailfish, usize) {
    let as_digit = chars[offset].to_digit(10);
    if let Some(d) = as_digit {
        return (Snailfish::Value(d), 1);
    }

    let (left, left_width) = parse_snailfish_pair(chars, offset + BRACE_WIDTH);
    let (right, consumed2) =
        parse_snailfish_pair(chars, offset + BRACE_WIDTH + left_width + COMMA_WIDTH);

    (
        Snailfish::Pair((Box::new(left), Box::new(right))),
        BRACE_WIDTH + left_width + COMMA_WIDTH + consumed2 + BRACE_WIDTH,
    )
}

fn parse_lines(lines: &[&str]) -> Vec<Snailfish> {
    let mut snailfish_pairs = Vec::new();
    for line in lines {
        snailfish_pairs.push(parse_snailfish_pair(&line.chars().collect::<Vec<_>>(), 0));
    }

    snailfish_pairs
        .into_iter()
        .map(|(s, _)| s)
        .collect::<Vec<_>>()
}

fn add_number_to_snailfish(snailfish: Snailfish, number: Option<u32>, side: Side) -> Snailfish {
    match number {
        Some(n) => match snailfish {
            Snailfish::Pair((l, r)) => match side {
                Side::Left => {
                    Snailfish::Pair((add_number_to_snailfish(*l, number, side).into(), r))
                },
                Side::Right => {
                    Snailfish::Pair((l, add_number_to_snailfish(*r, number, side).into()))
                },
            },

            Snailfish::Value(x) => Snailfish::Value(x + n),
        },
        None => snailfish,
    }
}

fn explode(snailfish: Snailfish, depth: u32, exploded: bool) -> ExplodedSnailfish {
    match snailfish {
        value @ Snailfish::Value(_) => ExplodedSnailfish::new(value, None, None, exploded),
        Snailfish::Pair((l, r)) => {
            if depth == 4 {
                if exploded {
                    return ExplodedSnailfish::new(Snailfish::Pair((l, r)), None, None, true);
                }
                return ExplodedSnailfish::new(
                    Snailfish::Value(0),
                    l.extract_value(),
                    r.extract_value(),
                    true,
                );
            }

            let l_exploded = explode(*l, depth + 1, exploded);
            let r_exploded = explode(*r, depth + 1, exploded || l_exploded.exploded);

            if r_exploded.push_to_left.is_some() && l_exploded.push_to_right.is_some() {
                panic!();
            }

            let new_fish = Snailfish::Pair((
                add_number_to_snailfish(l_exploded.snailfish, r_exploded.push_to_left, Side::Right)
                    .into(),
                add_number_to_snailfish(r_exploded.snailfish, l_exploded.push_to_right, Side::Left)
                    .into(),
            ));

            ExplodedSnailfish::new(
                new_fish,
                l_exploded.push_to_left,
                r_exploded.push_to_right,
                exploded || l_exploded.exploded || r_exploded.exploded,
            )
        },
    }
}

fn split(snailfish: Snailfish, did1split: bool) -> SplitSnailfish {
    match snailfish {
        Snailfish::Pair((l, r)) => {
            let l_split = split(*l, did1split);
            let r_split = split(*r, did1split || l_split.split);

            SplitSnailfish::new(
                Snailfish::Pair((l_split.snailfish.into(), r_split.snailfish.into())),
                did1split || l_split.split || r_split.split,
            )
        },

        Snailfish::Value(x) => {
            if x >= 10 && !did1split {
                let left = x / 2;
                let right = x - left;
                SplitSnailfish::new(
                    Snailfish::Pair((
                        Snailfish::Value(left).into(),
                        Snailfish::Value(right).into(),
                    )),
                    true,
                )
            } else {
                SplitSnailfish::new(Snailfish::Value(x), did1split)
            }
        },
    }
}

fn sum_explode_split(input: Vec<Snailfish>) -> Snailfish {
    let reduced = input.into_iter().reduce(|acc, item| {
        println!("Adding {:?} and {:?}", acc, item);
        let mut snailfish_sum = Snailfish::Pair((acc.into(), item.into()));
        println!("after addition: {:?}", snailfish_sum);

        let mut stable = false;

        while !stable {
            stable = true;

            let mut explosion_stable = false;

            // explode until no more explosions
            while !explosion_stable {
                explosion_stable = true;

                let esf = explode(snailfish_sum, 0, false);

                snailfish_sum = esf.snailfish;

                if esf.exploded {
                    println!("after explode:  {:?}", &snailfish_sum);

                    // if we had an explosion we have to run again
                    explosion_stable = false;
                    stable = false;
                }

                if explosion_stable {
                    break;
                }
            }

            // only split ONCE, then explode again
            let ssf = split(snailfish_sum, false);

            snailfish_sum = ssf.snailfish;

            if ssf.split {
                println!("after split:    {:?}", &snailfish_sum);

                stable = false;
            }

            if stable {
                break;
            }
        }

        snailfish_sum
    });

    reduced.unwrap()
}

fn calculate_magnitude(snailfish: &Snailfish) -> u32 {
    match snailfish {
        Snailfish::Value(x) => *x,
        Snailfish::Pair((l, r)) => 3 * calculate_magnitude(l) + 2 * calculate_magnitude(r),
    }
}

fn find_combination_with_higest_magnitude(
    snailfish: &[Snailfish],
) -> (u32, Snailfish, Snailfish, Snailfish) {
    let mut max = u32::MIN;
    let mut left = None;
    let mut right = None;
    let mut result = None;

    for snailfish_left in snailfish {
        for snailfish_right in snailfish {
            let temp_result =
                sum_explode_split(vec![snailfish_left.clone(), snailfish_right.clone()]);

            let magnitude = calculate_magnitude(&temp_result);

            if magnitude > max {
                max = magnitude;

                left = Some(snailfish_left.clone());
                right = Some(snailfish_right.clone());
                result = Some(temp_result);
            }
        }
    }

    (max, left.unwrap(), right.unwrap(), result.unwrap())
}
pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let snailfish = parse_lines(&lines);

        let result = sum_explode_split(snailfish);

        let magnitude = calculate_magnitude(&result);

        PartSolution::U32(magnitude)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let snailfish = parse_lines(&lines);

        PartSolution::U32(find_combination_with_higest_magnitude(&snailfish).0)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {

        use crate::{
            day_18::{
                calculate_magnitude, explode, parse_lines, parse_snailfish_pair, split,
                sum_explode_split, Snailfish, Solution,
            },
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(4243));
        }

        #[test]
        fn parse_example_1() {
            let str = "[1,2]".to_owned();

            let expected =
                Snailfish::Pair((Box::new(Snailfish::Value(1)), Box::new(Snailfish::Value(2))));

            let (result, _) = parse_snailfish_pair(&str.chars().collect::<Vec<_>>(), 0);

            assert_eq!(expected, result);
        }

        #[test]
        fn parse_example_2() {
            let str = "[[1,2],3]".to_owned();

            let expected = Snailfish::Pair((
                Box::new(Snailfish::Pair((
                    Box::new(Snailfish::Value(1)),
                    Box::new(Snailfish::Value(2)),
                ))),
                Box::new(Snailfish::Value(3)),
            ));

            let (result, _) = parse_snailfish_pair(&str.chars().collect::<Vec<_>>(), 0);

            assert_eq!(expected, result);
        }

        #[test]
        fn parse_example_3() {
            let str = "[9,[8,7]]".to_owned();

            let expected = Snailfish::Pair((
                Box::new(Snailfish::Value(9)),
                Box::new(Snailfish::Pair((
                    Box::new(Snailfish::Value(8)),
                    Box::new(Snailfish::Value(7)),
                ))),
            ));

            let (result, _) = parse_snailfish_pair(&str.chars().collect::<Vec<_>>(), 0);

            assert_eq!(expected, result);
        }

        #[test]
        fn parse_example_4() {
            let str = "[[1,9],[8,5]]".to_owned();

            let expected = Snailfish::Pair((
                Box::new(Snailfish::Pair((
                    Box::new(Snailfish::Value(1)),
                    Box::new(Snailfish::Value(9)),
                ))),
                Box::new(Snailfish::Pair((
                    Box::new(Snailfish::Value(8)),
                    Box::new(Snailfish::Value(5)),
                ))),
            ));

            let (result, _) = parse_snailfish_pair(&str.chars().collect::<Vec<_>>(), 0);

            assert_eq!(expected, result);
        }

        #[test]
        fn parse_example_5() {
            let str = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]".to_owned();

            let expected = Snailfish::Pair((
                Box::new(Snailfish::Pair((
                    Box::new(Snailfish::Pair((
                        Box::new(Snailfish::Pair((
                            Box::new(Snailfish::Value(1)),
                            Box::new(Snailfish::Value(2)),
                        ))),
                        Box::new(Snailfish::Pair((
                            Box::new(Snailfish::Value(3)),
                            Box::new(Snailfish::Value(4)),
                        ))),
                    ))),
                    Box::new(Snailfish::Pair((
                        Box::new(Snailfish::Pair((
                            Box::new(Snailfish::Value(5)),
                            Box::new(Snailfish::Value(6)),
                        ))),
                        Box::new(Snailfish::Pair((
                            Box::new(Snailfish::Value(7)),
                            Box::new(Snailfish::Value(8)),
                        ))),
                    ))),
                ))),
                Box::new(Snailfish::Value(9)),
            ));

            let (result, _) = parse_snailfish_pair(&str.chars().collect::<Vec<_>>(), 0);

            assert_eq!(expected, result);
        }

        #[test]
        fn parse_example_6() {
            let str = "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]".to_owned();

            let expected = Snailfish::Pair((
                Box::new(Snailfish::Pair((
                    Box::new(Snailfish::Pair((
                        Box::new(Snailfish::Value(9)),
                        Box::new(Snailfish::Pair((
                            Box::new(Snailfish::Value(3)),
                            Box::new(Snailfish::Value(8)),
                        ))),
                    ))),
                    Box::new(Snailfish::Pair((
                        Box::new(Snailfish::Pair((
                            Box::new(Snailfish::Value(0)),
                            Box::new(Snailfish::Value(9)),
                        ))),
                        Box::new(Snailfish::Value(6)),
                    ))),
                ))),
                Box::new(Snailfish::Pair((
                    Box::new(Snailfish::Pair((
                        Box::new(Snailfish::Pair((
                            Box::new(Snailfish::Value(3)),
                            Box::new(Snailfish::Value(7)),
                        ))),
                        Box::new(Snailfish::Pair((
                            Box::new(Snailfish::Value(4)),
                            Box::new(Snailfish::Value(9)),
                        ))),
                    ))),
                    Box::new(Snailfish::Value(3)),
                ))),
            ));

            let (result, _) = parse_snailfish_pair(&str.chars().collect::<Vec<_>>(), 0);

            assert_eq!(expected, result);
        }

        #[test]
        fn parse_explode_1() {
            let unparsed_input = "[[[[[9,8],1],2],3],4]".to_owned();
            let unparsed_expected = "[[[[0,9],2],3],4]".to_owned();

            let (input, _) = parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);
            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let exploded = explode(input, 0, false);

            assert_eq!(expected, exploded.snailfish);
        }

        #[test]
        fn parse_explode_2() {
            let unparsed_input = "[7,[6,[5,[4,[3,2]]]]]".to_owned();
            let unparsed_expected = "[7,[6,[5,[7,0]]]]".to_owned();

            let (input, _) = parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);
            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let exploded = explode(input, 0, false);

            assert_eq!(expected, exploded.snailfish);
        }

        #[test]
        fn parse_explode_3() {
            let unparsed_input = "[[6,[5,[4,[3,2]]]],1]".to_owned();
            let unparsed_expected = "[[6,[5,[7,0]]],3]".to_owned();

            let (input, _) = parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);
            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let exploded = explode(input, 0, false);

            assert_eq!(expected, exploded.snailfish);
        }

        #[test]
        fn parse_explode_4() {
            let unparsed_input = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_owned();
            let unparsed_expected = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".to_owned();

            let (input, _) = parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);
            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let exploded = explode(input, 0, false);

            assert_eq!(expected, exploded.snailfish);
        }

        #[test]
        fn parse_explode_5() {
            let unparsed_input = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".to_owned();
            let unparsed_expected = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_owned();

            let (input, _) = parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);
            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let exploded = explode(input, 0, false);

            assert_eq!(expected, exploded.snailfish);
        }

        #[test]
        fn full_example_1() {
            let unparsed_input_1 = "[[[[4,3],4],4],[7,[[8,4],9]]]".to_owned();
            let unparsed_input_2 = "[1,1]".to_owned();
            let unparsed_expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_owned();

            let (input_1, _) =
                parse_snailfish_pair(&unparsed_input_1.chars().collect::<Vec<_>>(), 0);
            let (input_2, _) =
                parse_snailfish_pair(&unparsed_input_2.chars().collect::<Vec<_>>(), 0);

            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let sum = Snailfish::Pair((input_1.into(), input_2.into()));

            let exploded_first = explode(sum, 0, false);
            let exploded_second = explode(exploded_first.snailfish, 0, false);
            let split_first = split(exploded_second.snailfish, false);
            let split_second = split(split_first.snailfish, false);
            let after_explode = explode(split_second.snailfish, 0, false);

            assert_eq!(expected, after_explode.snailfish);
        }

        #[test]
        fn full_example_1_automated() {
            let unparsed_input_1 = "[[[[4,3],4],4],[7,[[8,4],9]]]".to_owned();
            let unparsed_input_2 = "[1,1]".to_owned();
            let unparsed_expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_owned();

            let (input_1, _) =
                parse_snailfish_pair(&unparsed_input_1.chars().collect::<Vec<_>>(), 0);
            let (input_2, _) =
                parse_snailfish_pair(&unparsed_input_2.chars().collect::<Vec<_>>(), 0);

            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let result = sum_explode_split(vec![input_1, input_2]);

            assert_eq!(expected, result);
        }

        #[test]
        fn full_example_final_sum_1() {
            let unparsed_input_s = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]"];
            let unparsed_expected = "[[[[1,1],[2,2]],[3,3]],[4,4]]".to_owned();

            let snailfish = parse_lines(&unparsed_input_s);

            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let result = sum_explode_split(snailfish);

            assert_eq!(expected, result);
        }

        #[test]
        fn full_example_final_sum_2() {
            let unparsed_input_s = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"];
            let unparsed_expected = "[[[[3,0],[5,3]],[4,4]],[5,5]]".to_owned();

            let snailfish = parse_lines(&unparsed_input_s);

            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let result = sum_explode_split(snailfish);

            assert_eq!(expected, result);
        }

        #[test]
        fn full_example_final_sum_3() {
            let unparsed_input_s = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"];
            let unparsed_expected = "[[[[5,0],[7,4]],[5,5]],[6,6]]".to_owned();

            let snailfish = parse_lines(&unparsed_input_s);

            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let result = sum_explode_split(snailfish);

            assert_eq!(expected, result);
        }

        #[test]
        fn slightly_larger_example_1() {
            let unparsed_input_s = vec![
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[2,9]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[4,2],2],6],[8,7]]",
            ];

            let unparsed_expected =
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_owned();

            let snailfish = parse_lines(&unparsed_input_s);

            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let result = sum_explode_split(snailfish);

            assert_eq!(expected, result);
        }

        #[test]
        fn magnitude_1() {
            let unparsed_input = "[9,1]".to_owned();

            let (snailfish, _) =
                parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);

            let expected = 29;

            let result = calculate_magnitude(&snailfish);

            assert_eq!(expected, result);
        }

        #[test]
        fn magnitude_2() {
            let unparsed_input = "[1,9]".to_owned();

            let (snailfish, _) =
                parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);

            let expected = 21;

            let result = calculate_magnitude(&snailfish);

            assert_eq!(expected, result);
        }

        #[test]
        fn magnitude_3() {
            let unparsed_input = "[[9,1],[1,9]]".to_owned();

            let (snailfish, _) =
                parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);

            let expected = 129;

            let result = calculate_magnitude(&snailfish);

            assert_eq!(expected, result);
        }

        #[test]
        fn magnitude_4() {
            let unparsed_input = "[[1,2],[[3,4],5]]".to_owned();

            let (snailfish, _) =
                parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);

            let expected = 143;

            let result = calculate_magnitude(&snailfish);

            assert_eq!(expected, result);
        }

        #[test]
        fn magnitude_5() {
            let unparsed_input = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_owned();

            let (snailfish, _) =
                parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);

            let expected = 1384;

            let result = calculate_magnitude(&snailfish);

            assert_eq!(expected, result);
        }
        #[test]
        fn magnitude_6() {
            let unparsed_input = "[[[[1,1],[2,2]],[3,3]],[4,4]]".to_owned();

            let (snailfish, _) =
                parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);

            let expected = 445;

            let result = calculate_magnitude(&snailfish);

            assert_eq!(expected, result);
        }
        #[test]
        fn magnitude_7() {
            let unparsed_input = "[[[[3,0],[5,3]],[4,4]],[5,5]]".to_owned();

            let (snailfish, _) =
                parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);

            let expected = 791;

            let result = calculate_magnitude(&snailfish);

            assert_eq!(expected, result);
        }
        #[test]
        fn magnitude_8() {
            let unparsed_input = "[[[[5,0],[7,4]],[5,5]],[6,6]]".to_owned();

            let (snailfish, _) =
                parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);

            let expected = 1137;

            let result = calculate_magnitude(&snailfish);

            assert_eq!(expected, result);
        }
        #[test]
        fn magnitude_9() {
            let unparsed_input = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_owned();

            let (snailfish, _) =
                parse_snailfish_pair(&unparsed_input.chars().collect::<Vec<_>>(), 0);

            let expected = 3488;

            let result = calculate_magnitude(&snailfish);

            assert_eq!(expected, result);
        }

        #[test]
        fn example_homework_assignment() {
            let unparsed_input_s = vec![
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
                "[[[5,[2,8]],4],[5,[[9,9],0]]]",
                "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
                "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
                "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
                "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
                "[[[[5,4],[7,7]],8],[[8,3],8]]",
                "[[9,3],[[9,9],[6,[4,9]]]]",
                "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
                "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
            ];

            let unparsed_expected =
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".to_owned();

            let snailfish = parse_lines(&unparsed_input_s);

            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let result = sum_explode_split(snailfish);

            assert_eq!(expected, result);

            let magnitude = calculate_magnitude(&result);

            let expected_magnitude = 4140;

            assert_eq!(expected_magnitude, magnitude);
        }
    }

    mod part_2 {

        use crate::{
            day_18::{
                find_combination_with_higest_magnitude, parse_lines, parse_snailfish_pair, Solution,
            },
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U32(4701));
        }

        #[test]
        fn example_homework_assignment() {
            let unparsed_input_s = vec![
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
                "[[[5,[2,8]],4],[5,[[9,9],0]]]",
                "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
                "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
                "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
                "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
                "[[[[5,4],[7,7]],8],[[8,3],8]]",
                "[[9,3],[[9,9],[6,[4,9]]]]",
                "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
                "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
            ];

            let snailfish = parse_lines(&unparsed_input_s);

            let unparsed_expected =
                "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]".to_owned();

            let (expected, _) =
                parse_snailfish_pair(&unparsed_expected.chars().collect::<Vec<_>>(), 0);

            let unparsed_expected_left = "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]".to_owned();

            let (expected_left, _) =
                parse_snailfish_pair(&unparsed_expected_left.chars().collect::<Vec<_>>(), 0);

            let unparsed_expected_right =
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]".to_owned();

            let (expected_right, _) =
                parse_snailfish_pair(&unparsed_expected_right.chars().collect::<Vec<_>>(), 0);

            let (magnitude, left, right, result) =
                find_combination_with_higest_magnitude(&snailfish);

            let expected_magnitude = 3993;

            assert_eq!(expected_magnitude, magnitude);
            assert_eq!(expected, result);
            assert_eq!(expected_left, left);
            assert_eq!(expected_right, right);
        }
    }
}
