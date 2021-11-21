use crate::error::AoCError;

// https://adventofcode.com/2020/day/2
pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    Err(Box::new(AoCError {
        message: "Oops".into(),
    }))
}

#[test]
fn outcome() {
    // assert_eq!(1478615040, find_solution().unwrap());
}
