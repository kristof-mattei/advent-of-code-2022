fn parse_lines(lines: &[String]) -> Vec<u32> {
    lines.iter().map(|s| s.parse::<u32>().unwrap()).collect()
}

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

pub fn find_solution() -> u32 {
    let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

    let numbers = parse_lines(&lines);

    count_window_of_3_increments(&numbers)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(1748, find_solution());
    }

    #[test]
    fn example() {
        let t = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_window_of_3_increments(&t), 5);
    }
}
