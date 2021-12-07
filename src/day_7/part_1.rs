use std::cmp;

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

fn find_minimum(crabs: &[u32]) -> u32 {
    let minimum_depth = *crabs.iter().min().unwrap();
    let max_depth = *crabs.iter().max().unwrap();

    let mut min = u32::MAX;

    for depth in minimum_depth..=max_depth {
        let mut fuel_needed_for_depth: u32 = 0;

        for crab in crabs {
            let fuel_needed = cmp::max(*crab, depth) - cmp::min(*crab, depth);

            fuel_needed_for_depth += fuel_needed;

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
        assert_eq!(find_solution(), 339_321);
    }

    #[test]
    fn playground() {
        let lines: Vec<String> = vec!["16,1,2,0,4,2,7,1,2,14"]
            .iter()
            .map(ToString::to_string)
            .collect();

        let crabs: Vec<u32> = parse_lines(&lines);

        assert_eq!(37, find_minimum(&crabs));
    }
}
