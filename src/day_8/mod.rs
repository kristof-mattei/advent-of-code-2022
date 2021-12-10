use std::{collections::HashSet, ops::Sub};

use crate::shared::{Day, PartSolution};

fn parse_lines(input: &[String]) -> Vec<Vec<LetterCombination>> {
    let mut into_pieces = Vec::new();

    for line in input {
        let pieces = line
            .split(' ')
            .filter(|x| *x != "|")
            .map(|f| HashSet::from_iter(f.trim().chars().collect::<LetterCombination>()))
            .collect();

        into_pieces.push(pieces);
    }

    into_pieces
}

type LetterCombination = HashSet<char>;

fn count_digits_1_4_7_8(input: &[Vec<LetterCombination>]) -> usize {
    let mut digits = Vec::new();

    for vecs in input {
        for unparsed_number in vecs {
            let number: Option<u32> = match unparsed_number.len() {
                2 => Some(1),
                3 => Some(7),
                4 => Some(4),
                7 => Some(8),
                _ => None,
            };

            if let Some(x) = number {
                digits.push(x);
            }
        }
    }

    digits.len()
}

fn single_piece(input: &HashSet<char>) -> char {
    *input.iter().next().unwrap()
}

fn calculate_signal_patterns(x: &[LetterCombination]) -> u32 {
    let input: Vec<&LetterCombination> = x.iter().take(10).collect();

    // these are fixed by length
    let one: &LetterCombination = input.iter().find(|x| x.len() == 2).unwrap();
    let seven: &LetterCombination = input.iter().find(|x| x.len() == 3).unwrap();
    let four: &LetterCombination = input.iter().find(|x| x.len() == 4).unwrap();
    let eight: &LetterCombination = input.iter().find(|x| x.len() == 7).unwrap();

    assert_eq!(one.len(), 2);
    assert_eq!(seven.len(), 3);
    assert_eq!(four.len(), 4);
    assert_eq!(eight.len(), 7);

    // these are all a length of six
    let zero_six_nine: Vec<&LetterCombination> =
        input.iter().filter(|x| x.len() == 6).copied().collect();

    assert_eq!(zero_six_nine.len(), 3);

    // 7 - 1 reveals the top part
    //  _          _
    //   | -   | =
    //   |     |
    let top = seven.sub(one);

    assert_eq!(top.len(), 1);

    // 4 - 7 reveals the top and middle part
    //        _
    // |_| -   | = |_
    //   |     |
    let middle_and_left_top: LetterCombination = four.sub(seven);

    assert_eq!(middle_and_left_top.len(), 2);

    // 6 and 9 both contain the top and middle part, unlike zero, which is missing the middle part
    let six_nine: Vec<&LetterCombination> = zero_six_nine
        .iter()
        .filter(|x| x.is_superset(&middle_and_left_top))
        .copied()
        .collect();

    assert_eq!(six_nine.len(), 2);

    // zero is what is in zero_six_nine after we remove six and nine
    let zero: &LetterCombination = zero_six_nine
        .iter()
        .find(|x| !six_nine.contains(x))
        .unwrap();

    assert_eq!(zero.len(), 6);

    // 8 - 0 reveals middle part
    //  _     _
    // |_| - | | = _
    // |_|   |_|
    let middle = eight.sub(zero);

    assert_eq!(middle.len(), 1);

    // (left top and middle) - middle reveals the left top
    //
    // |_  -  _  =  |
    //
    let left_top = middle_and_left_top.sub(&middle);

    assert_eq!(left_top.len(), 1);

    // 9 is the one from 6 and 9 that completely encompasses 1
    let nine: &LetterCombination = six_nine.iter().find(|x| x.is_superset(one)).unwrap();

    assert_eq!(nine.len(), 6);

    // 6 is the one that isn't nine
    let six: &LetterCombination = six_nine.iter().find(|x| **x != nine).unwrap();

    assert_eq!(six.len(), 6);

    // 1 - 6 reveals the right top part
    //        _
    //   | - |_  =   |
    //   |   |_|
    let right_top = one.sub(six);

    assert_eq!(right_top.len(), 1);

    // 8 - 9 reveals the left bottom part
    //  _     _
    // |_| - |_| =
    // |_|    _|   |
    let left_bottom = eight.sub(nine);

    // the intersection of 6 and 1 is the right bottom piece
    //  _
    // |_  -   | =
    // |_|     |     |
    let right_bottom = six.intersection(one).copied().collect();

    let bottom = nine
        .sub(&top)
        .sub(&middle)
        .sub(&left_top)
        .sub(&right_top)
        .sub(&right_bottom);

    assert_eq!(bottom.len(), 1);

    let top_part = single_piece(&top);
    let left_top_part = single_piece(&left_top);
    let right_top_part = single_piece(&right_top);
    let middle_part = single_piece(&middle);
    let left_bottom_part = single_piece(&left_bottom);
    let right_bottom_part = single_piece(&right_bottom);
    let bottom_part = single_piece(&bottom);

    println!(
        "     {}{}{}{}\n{}    {}\n{}    {}\n{}{}{}{}\n{}    {}\n{}    {}\n{}{}{}{}",
        top_part,
        top_part,
        top_part,
        top_part,
        left_top_part,
        right_top_part,
        left_top_part,
        right_top_part,
        middle_part,
        middle_part,
        middle_part,
        middle_part,
        left_bottom_part,
        right_bottom_part,
        left_bottom_part,
        right_bottom_part,
        bottom_part,
        bottom_part,
        bottom_part,
        bottom_part
    );

    let two: HashSet<char> = HashSet::from_iter(vec![
        top_part,
        right_top_part,
        middle_part,
        left_bottom_part,
        bottom_part,
    ]);

    let three: HashSet<char> = HashSet::from_iter(vec![
        top_part,
        right_top_part,
        middle_part,
        right_bottom_part,
        bottom_part,
    ]);

    let five: HashSet<char> = HashSet::from_iter(vec![
        top_part,
        left_top_part,
        middle_part,
        right_bottom_part,
        bottom_part,
    ]);

    let key = vec![
        zero, one, &two, &three, four, &five, six, seven, eight, nine,
    ];

    let encoded_solution: Vec<&LetterCombination> = x.iter().skip(10).collect();

    decode_solution(&key, &encoded_solution)
}

fn decode_solution(key: &[&LetterCombination], numbers: &[&LetterCombination]) -> u32 {
    let mut result = 0;

    // to calculate the power
    let length = numbers.len() - 1;

    for (index, n) in numbers.iter().enumerate() {
        let decoded = key.iter().position(|x| x == n).unwrap();

        // now based on the index we need to add it to result
        // letter at index zero is actually the higest
        result += 10_usize.pow(length as u32 - index as u32) * decoded;
    }

    result as u32
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let split = parse_lines(&lines);

        let last_4: Vec<Vec<LetterCombination>> = split
            .iter()
            .map(|l| {
                l.iter()
                    .skip(l.len() - 4)
                    .map(|x: &LetterCombination| x.iter().copied().collect())
                    .collect()
            })
            .collect();

        PartSolution::USize(count_digits_1_4_7_8(&last_4))
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let mut last_4_sum = Vec::new();

        let split = parse_lines(&lines);

        for s in split {
            let decoded = calculate_signal_patterns(&s);

            last_4_sum.push(decoded);
        }

        PartSolution::U32(last_4_sum.iter().sum())
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
            day_8::{count_digits_1_4_7_8, parse_lines, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::USize(392));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let split = parse_lines(&lines);

            assert_eq!(26, count_digits_1_4_7_8(&split));
        }
    }

    mod part_2 {
        use crate::{
            day_8::{calculate_signal_patterns, parse_lines, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U32(1_004_688));
        }

        #[test]
        fn example() {
            let lines = get_example();

            let split = parse_lines(&lines);

            let mut sum = 0;

            for s in split {
                let decoded = calculate_signal_patterns(&s);

                sum += decoded;
            }

            assert_eq!(61229, sum);
        }
    }
}
