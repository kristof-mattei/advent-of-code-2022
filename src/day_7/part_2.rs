use std::cmp;
use std::collections::HashMap;

fn parse_lines(lines: &[String]) -> Vec<u32> {
    lines[0]
        .split(',')
        .map(|f| f.parse::<u32>().unwrap())
        .collect()
}

pub fn find_solution() -> u32 {
    let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

    let crabs = parse_lines(&lines);

    find_minimum(&crabs)
}

fn calculate_fuel_needed(steps: u32, cache: &mut HashMap<u32, u32>) -> u32 {
    // cache distances
    if let Some(fuel_needed) = cache.get(&steps) {
        *fuel_needed
    } else {
        let mut fuel_needed: u32 = 0;
        for i in 1..=steps {
            fuel_needed += i;
        }

        cache.insert(steps, fuel_needed);
        fuel_needed
    }
}

fn find_minimum(crabs: &[u32]) -> u32 {
    let minimum_depth = *crabs.iter().min().unwrap();
    let max_depth = *crabs.iter().max().unwrap();

    let mut min = u32::MAX;

    let mut fuel_needed_cache = HashMap::<u32, u32>::new();

    for depth in minimum_depth..=max_depth {
        let mut fuel_needed_for_depth: u32 = 0;

        for crab in crabs {
            let crab_distance_from_depth = cmp::max(*crab, depth) - cmp::min(*crab, depth);

            fuel_needed_for_depth +=
                calculate_fuel_needed(crab_distance_from_depth, &mut fuel_needed_cache);

            // no need to continue if we are already over the last minimum
            if fuel_needed_for_depth > min {
                break;
            }
        }

        if fuel_needed_for_depth < min {
            min = fuel_needed_for_depth;
        }
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(find_solution(), 95_476_244);
    }

    #[test]
    fn playground() {
        let lines: Vec<String> = vec!["16,1,2,0,4,2,7,1,2,14"]
            .iter()
            .map(ToString::to_string)
            .collect();

        let crabs: Vec<u32> = parse_lines(&lines);

        assert_eq!(168, find_minimum(&crabs));
    }

    #[test]
    fn fuel_needed() {
        assert_eq!(1, calculate_fuel_needed(1, &mut HashMap::new()));
        assert_eq!(3, calculate_fuel_needed(2, &mut HashMap::new()));
        assert_eq!(15, calculate_fuel_needed(5, &mut HashMap::new()));
    }
}
