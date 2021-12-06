fn parse_lines(lines: &[String]) -> Vec<u32> {
    lines.iter().map(|s| s.parse::<u32>().unwrap()).collect()
}

pub fn count_increments(list: &[u32]) -> u32 {
    let mut count = 0;
    for i in list.windows(2) {
        if i[1] > i[0] {
            count += 1;
        }
    }

    count
}

pub fn find_solution() -> u32 {
    let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

    let numbers = parse_lines(&lines);

    count_increments(&numbers)
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(1722, find_solution());
    }

    #[test]
    fn playground() {
        let t = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_increments(&t), 7);
    }
}
