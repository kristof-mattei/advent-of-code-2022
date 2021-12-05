use crate::utils::read_file;

enum Common {
    Least,
    Most,
}

fn find_common(lines: &[u32], bit_index: usize, common: &Common) -> usize {
    let mut bits = Vec::new();

    for line in lines {
        bits.push((line >> bit_index) & 0b01);
    }

    let count_of_1 = bits.iter().filter(|x| **x == 1).count();
    let count_of_0 = bits.iter().filter(|x| **x == 0).count();

    match common {
        Common::Least => {
            if count_of_1 >= count_of_0 {
                0
            } else {
                1
            }
        }
        Common::Most => {
            if count_of_1 >= count_of_0 {
                1
            } else {
                0
            }
        }
    }
}

fn move_and_reduce(
    filtered_lines: &[u32],
    bits_to_consider: usize,
    bit_index: usize,
    common: &Common,
) -> Vec<u32> {
    if filtered_lines.len() == 1 {
        return filtered_lines.to_vec();
    }

    let mut new_set: Vec<u32> = Vec::new();

    let most_or_least_common =
        find_common(filtered_lines, bits_to_consider - bit_index - 1, common);

    for line in filtered_lines {
        if (line >> (bits_to_consider - bit_index - 1) & 0b01) == most_or_least_common as u32 {
            new_set.push(*line);
        }
    }

    move_and_reduce(&new_set, bits_to_consider, bit_index + 1, common)
}

fn parse_diagnostic_report(diagnostic_report_lines: &[u32], bits_to_consider: usize) -> (u32, u32) {
    let oxygen = move_and_reduce(diagnostic_report_lines, bits_to_consider, 0, &Common::Most);
    let co2 = move_and_reduce(diagnostic_report_lines, bits_to_consider, 0, &Common::Least);

    (oxygen[0], co2[0])
}

fn parse_lines(lines: &[String]) -> (Vec<u32>, usize) {
    let mut parsed = Vec::new();

    for line in lines {
        parsed.push(u32::from_str_radix(line, 2).unwrap());
    }

    let bits_to_consider = match lines.get(0) {
        Some(l) => l.len(),
        None => 0,
    };

    (parsed, bits_to_consider)
}

pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let lines = read_file("./src/day_3/input.txt".into())?;

    let (diagnostic_report_lines, bits_to_consider) = parse_lines(&lines);

    let (oxygen, co2) = parse_diagnostic_report(&diagnostic_report_lines, bits_to_consider);

    Ok(oxygen * co2)
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(find_solution().unwrap(), 4125600);
    }

    #[test]
    fn playground() {
        let lines: Vec<String> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        let (instructions, bits_to_consider) = parse_lines(&lines);

        let (oxygen, co2) = parse_diagnostic_report(&instructions, bits_to_consider);

        assert_eq!(oxygen, 23);
        assert_eq!(co2, 10);
    }
}
