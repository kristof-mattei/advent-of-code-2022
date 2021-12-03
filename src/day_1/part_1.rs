use crate::utils::read_file;

pub fn count_increments(list: &[u32]) -> u32 {
    let mut count = 0;
    for i in list.windows(2) {
        if i[1] > i[0] {
            count += 1;
        }
    }

    count
}

pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let split = read_file("./src/day_1/input.txt".into())?;

    let numbers: Vec<u32> = split
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    Ok(count_increments(&numbers))
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn outcome() {
        assert!(matches!(find_solution(), Ok(1722)));
    }

    #[test]
    fn playground() {
        let t = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_increments(&t), 7);
    }
}
