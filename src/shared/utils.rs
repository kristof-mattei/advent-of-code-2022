#[macro_export]
#[expect(clippy::crate_in_macro_def, reason = "We do want the parent one")]
macro_rules! test_part_1 {
    ($value:literal) => {{
        use advent_of_code_2022::shared::Parts;
        use advent_of_code_2022::shared::solution::read_file;
        use pretty_assertions::assert_eq;

        use crate::{DAY, Solution};

        if std::env::var("CI").is_err() {
            assert_eq!($value, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }
    }};
}

#[macro_export]
#[expect(clippy::crate_in_macro_def, reason = "We do want the parent one")]
macro_rules! test_part_2 {
    ($value:literal) => {{
        use advent_of_code_2022::shared::Parts;
        use advent_of_code_2022::shared::solution::read_file;
        use pretty_assertions::assert_eq;

        use crate::{DAY, Solution};

        if std::env::var("CI").is_err() {
            assert_eq!($value, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }
    }};
}

#[macro_export]
#[expect(clippy::crate_in_macro_def, reason = "We do want the parent one")]
macro_rules! test_example_part_1 {
    ($value:literal, $part:literal) => {{
        use advent_of_code_2022::shared::Parts;
        use advent_of_code_2022::shared::solution::read_file_part;
        use pretty_assertions::assert_eq;

        use crate::{DAY, Solution};

        assert_eq!(
            $value,
            (Solution {}).part_1(&read_file_part("examples", &DAY, $part))
        );
    }};
    ($value:literal) => {{
        use advent_of_code_2022::shared::Parts;
        use advent_of_code_2022::shared::solution::read_file;
        use pretty_assertions::assert_eq;

        use crate::{DAY, Solution};

        assert_eq!($value, (Solution {}).part_1(&read_file("examples", &DAY)));
    }};
}

#[macro_export]
#[expect(clippy::crate_in_macro_def, reason = "We do want the parent one")]
macro_rules! test_example_part_2 {
    ($value:literal, $part:literal) => {{
        use advent_of_code_2022::shared::Parts;
        use advent_of_code_2022::shared::solution::read_file_part;
        use pretty_assertions::assert_eq;

        use crate::{DAY, Solution};

        assert_eq!(
            $value,
            (Solution {}).part_2(&read_file_part("examples", &DAY, $part))
        );
    }};
    ($value:literal) => {{
        use advent_of_code_2022::shared::Parts;
        use advent_of_code_2022::shared::solution::read_file;
        use pretty_assertions::assert_eq;

        use crate::{DAY, Solution};

        assert_eq!($value, (Solution {}).part_2(&read_file("examples", &DAY)));
    }};
}
