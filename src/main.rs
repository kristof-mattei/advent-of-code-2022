#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::let_and_return)]

mod shared;
mod utils;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn print_answer(day: u32, part: u32, result: &str) {
    println!("Answer to Day {}, part {} is ... {}", day, part, result);
}

fn main() {
    let solutions = vec![
        (
            day_1::part_1::find_solution().to_string(),
            day_1::part_2::find_solution().to_string(),
        ),
        (
            day_2::part_1::find_solution().to_string(),
            day_2::part_2::find_solution().to_string(),
        ),
        (
            day_3::part_1::find_solution().to_string(),
            day_3::part_2::find_solution().to_string(),
        ),
        (
            day_4::part_1::find_solution().to_string(),
            day_4::part_2::find_solution().to_string(),
        ),
        (
            day_5::part_1::find_solution().to_string(),
            day_5::part_2::find_solution().to_string(),
        ),
    ];

    let mut day: u32 = 1;

    for (day_solution_part_1, day_solution_part_2) in solutions {
        print_answer(day, 1, &day_solution_part_1);
        print_answer(day, 2, &day_solution_part_2);

        day += 1;
    }
}
