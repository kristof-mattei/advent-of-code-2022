use crate::utils::read_file;

fn parse_diagnostic_report(diagnostic_report_lines: &[u32], bits_to_consider: usize) -> (u32, u32) {
    let mut set_count_by_column: Vec<u32> = vec![0; bits_to_consider];

    for line in diagnostic_report_lines {
        for i in 0..bits_to_consider {
            if (line >> i & 0b0001) == 1 {
                set_count_by_column[i as usize] += 1;
            }
        }
    }

    let half_amount_of_lines = diagnostic_report_lines.len() as u32 / 2;

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..bits_to_consider {
        if set_count_by_column[i as usize] > half_amount_of_lines {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    (gamma, epsilon)
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

    let (gamma, epsilon) = parse_diagnostic_report(&diagnostic_report_lines, bits_to_consider);

    Ok(gamma * epsilon)
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(find_solution().unwrap(), 4_160_394);
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

        let (gamma, epsilon) = parse_diagnostic_report(&instructions, bits_to_consider);

        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }
}
