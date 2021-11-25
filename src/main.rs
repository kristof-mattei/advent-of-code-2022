#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
use std::fmt;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod shared;
mod utils;

fn print_answer<T: fmt::Display, E: fmt::Display>(day: u32, part: u32, result: &Result<T, E>) {
    match result {
        Ok(r) => {
            println!("Answer to Day {}, part {} is ... {}", day, part, r);
        }
        Err(e) => {
            println!(
                "Whoops! Day {}, part {} gave an error of ... {}",
                day, part, e
            );
        }
    }
}
fn main() {
    let days = [
        (
            day_1::part_1::find_solution(),
            day_1::part_2::find_solution(),
        ),
        (
            day_2::part_1::find_solution(),
            day_2::part_2::find_solution(),
        ),
        (
            day_3::part_1::find_solution(),
            day_3::part_2::find_solution(),
        ),
        (
            day_4::part_1::find_solution(),
            day_4::part_2::find_solution(),
        ),
        (
            day_5::part_1::find_solution(),
            day_5::part_2::find_solution(),
        ),
        (
            day_6::part_1::find_solution(),
            day_6::part_2::find_solution(),
        ),
        (
            day_7::part_1::find_solution(),
            day_7::part_2::find_solution(),
        ),
        (
            day_8::part_1::find_solution(),
            day_8::part_2::find_solution(),
        ),
    ];

    for (i, (part1, part2)) in days.iter().enumerate() {
        let day_index = i as u32;

        print_answer(day_index + 1, 1, part1);
        print_answer(day_index + 1, 2, part2);
    }
}
