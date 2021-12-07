fn parse_lines(_lines: &[String]) -> Vec<u32> {
    vec![]
}

pub fn find_solution() -> u32 {
    let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

    let _parsed = parse_lines(&lines);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(find_solution(), 0);
    }

    #[test]
    fn playground() {}
}
