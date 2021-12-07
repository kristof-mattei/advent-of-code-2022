fn parse_lines(lines: &[String]) -> Vec<u32> {
    let fishes_nearby = lines[0]
        .split(',')
        .map(|f| f.parse::<u32>().unwrap())
        .collect();

    fishes_nearby
}

fn age_fishes(fishes: Vec<u32>) -> Vec<u32> {
    let mut new_fishes_to_append: usize = 0;

    let mut new_fishes = Vec::new();

    for fish in fishes {
        match fish {
            0 => {
                new_fishes.push(6);
                new_fishes_to_append += 1;
            }
            x => new_fishes.push(x - 1),
        }
    }

    new_fishes.resize(new_fishes.len() + new_fishes_to_append, 8);

    new_fishes
}

pub fn find_solution() -> usize {
    let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

    let mut fishes = parse_lines(&lines);
    for _ in 0..80 {
        fishes = age_fishes(fishes);
    }

    fishes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(find_solution(), 395_627);
    }

    #[test]
    fn playground() {
        let lines: Vec<String> = vec!["3,4,3,1,2"].iter().map(ToString::to_string).collect();

        let mut fishes: Vec<u32> = parse_lines(&lines);

        for i in 0..18 {
            fishes = age_fishes(fishes);
            println!("After day {}", (i + 1));
        }

        assert_eq!(26, fishes.len());
    }

    #[test]
    fn playground_2() {
        let lines: Vec<String> = vec!["3,4,3,1,2"].iter().map(ToString::to_string).collect();

        let mut fishes: Vec<u32> = parse_lines(&lines);

        for i in 0..80 {
            fishes = age_fishes(fishes);
            println!("After day {}", (i + 1));
        }

        assert_eq!(5934, fishes.len());
    }

    #[test]
    fn playground_3() {
        let lines: Vec<String> = vec!["3,4,3,1,2"].iter().map(ToString::to_string).collect();

        let mut fishes: Vec<u32> = parse_lines(&lines);

        for i in 0..256 {
            fishes = age_fishes(fishes);
            println!("After day {}", (i + 1));
        }

        assert_eq!(26_984_457_539, fishes.len());
    }
}
