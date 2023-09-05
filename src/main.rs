#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::cargo)]
#![deny(warnings)]
// exceptions
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::let_and_return)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::uninlined_format_args)]
#![deny(let_underscore_drop)]
#![deny(non_ascii_idents)]

use shared::Day;

mod shared;
mod utils;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;

fn print_answer(day: u32, part: u32, result: &str) {
    println!("Answer to Day {}, part {} is ... {}", day, part, result);
}

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    let mut day: u32 = 1;

    let solutions: Vec<Box<dyn Day>> = vec![
        Box::new(day_01::Solution {}),
        Box::new(day_02::Solution {}),
        Box::new(day_03::Solution {}),
        Box::new(day_04::Solution {}),
        Box::new(day_05::Solution {}),
        Box::new(day_06::Solution {}),
        Box::new(day_07::Solution {}),
        Box::new(day_08::Solution {}),
        Box::new(day_09::Solution {}),
        Box::new(day_10::Solution {}),
        Box::new(day_11::Solution {}),
        Box::new(day_12::Solution {}),
        Box::new(day_13::Solution {}),
        Box::new(day_14::Solution {}),
        Box::new(day_15::Solution {}),
        Box::new(day_16::Solution {}),
    ];

    for solution in solutions {
        print_answer(day, 1, &solution.part_1().to_string());
        print_answer(day, 2, &solution.part_2().to_string());

        day += 1;
    }

    Ok(())
}
