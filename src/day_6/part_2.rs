use std::collections::HashMap;

fn parse_lines(lines: &[String]) -> HashMap<u8, u64> {
    let fishes_nearby: Vec<u8> = lines[0]
        .split(',')
        .map(|f| f.parse::<u8>().unwrap())
        .collect();

    let mut fish_counts: HashMap<u8, u64> = HashMap::new();

    for fish in fishes_nearby {
        let fish_count = *fish_counts.get(&fish).unwrap_or(&0);

        fish_counts.insert(fish, fish_count + 1);
    }

    println!("{:?}", fish_counts);

    fish_counts
}

fn age_fishes(fishes: &mut HashMap<u8, u64>) {
    // fishes age 0 will be reset to age 6, and will spawn new fishes aged 8
    let fishes_which_will_spawn_new_fishes = *fishes.get(&0).unwrap_or(&0);

    for i in 1..=8 {
        let current = *fishes.get(&i).unwrap_or(&0);

        fishes.insert(i - 1, current);
    }

    // all fishes reset create 1 offspring
    fishes.insert(8, fishes_which_will_spawn_new_fishes);

    // and add the ones at age 0 back to the pool, but they start back at 6
    let entry_6 = fishes.entry(6).or_default();
    *entry_6 += fishes_which_will_spawn_new_fishes;
}

pub fn find_solution() -> u64 {
    let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

    let mut fishes = parse_lines(&lines);
    for _ in 1..=256 {
        age_fishes(&mut fishes);
    }

    fishes.iter().map(|(_, v)| v).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(find_solution(), 1_767_323_539_209);
    }

    #[test]
    fn playground() {
        let lines: Vec<String> = vec!["3,4,3,1,2"].iter().map(ToString::to_string).collect();

        let mut fishes = parse_lines(&lines);

        for i in 1..=18 {
            age_fishes(&mut fishes);
            println!("After day {}", i);
        }

        assert_eq!(26, fishes.iter().map(|(_, v)| v).sum::<u64>());
    }

    #[test]
    fn playground_2() {
        let lines: Vec<String> = vec!["3,4,3,1,2"].iter().map(ToString::to_string).collect();

        let mut fishes = parse_lines(&lines);

        for i in 1..=80 {
            age_fishes(&mut fishes);
            println!("After day {}", i);
        }

        assert_eq!(5934, fishes.iter().map(|(_, v)| v).sum::<u64>());
    }

    #[test]
    fn playground_3() {
        let lines: Vec<String> = vec!["3,4,3,1,2"].iter().map(ToString::to_string).collect();

        let mut fishes = parse_lines(&lines);

        for i in 1..=256 {
            age_fishes(&mut fishes);
            println!("After day {}", i);
        }

        assert_eq!(26_984_457_539, fishes.iter().map(|(_, v)| v).sum::<u64>());
    }
}
