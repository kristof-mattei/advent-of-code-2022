#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::let_and_return)]

use shared::Day;

mod shared;
mod utils;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

fn print_answer(day: u32, part: u32, result: &str) {
    println!("Answer to Day {}, part {} is ... {}", day, part, result);
}

fn main() {
    let mut day: u32 = 1;

    let solutions_2: Vec<Box<dyn Day>> = vec![
        Box::new(day_1::Solution {}),
        Box::new(day_2::Solution {}),
        Box::new(day_3::Solution {}),
        Box::new(day_4::Solution {}),
        Box::new(day_5::Solution {}),
        Box::new(day_6::Solution {}),
        Box::new(day_7::Solution {}),
        Box::new(day_8::Solution {}),
    ];

    for solution in solutions_2 {
        print_answer(day, 1, &solution.part_1().to_string());
        print_answer(day, 2, &solution.part_2().to_string());

        day += 1;
    }

    // for (day_solution_part_1, day_solution_part_2) in solutions {
    //     print_answer(day, 1, &day_solution_part_1);
    //     print_answer(day, 2, &day_solution_part_2);

    //     day += 1;
    // }
}
