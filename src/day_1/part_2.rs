use crate::utils::read_file;

pub fn count_window_of_3_increments(list: &[u32]) -> u32 {
    let mut count = 0;
    let mut previous_window: u32 = 0;

    for i in list.windows(3) {
        let current_window: u32 = i.iter().sum();

        if current_window > previous_window {
            count += 1;
        }

        previous_window = current_window;
    }

    count - 1
}

pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_1/input.txt".into())?;

    let numbers: Vec<u32> = split
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    Ok(count_window_of_3_increments(&numbers))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn outcome() {
        assert!(matches!(find_solution(), Ok(1748)));
    }

    #[test]
    fn example() {
        let t = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_window_of_3_increments(&t), 5);
    }
}
