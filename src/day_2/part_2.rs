use crate::utils::read_file;

fn determine_direction(directions: &[Direction]) -> (u32, u32) {
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;

    for direction in directions {
        match direction {
            Direction::Up(x) => aim -= x,
            Direction::Down(x) => aim += x,
            Direction::Forward(x) => {
                forward += x;
                depth += aim * x;
            }
        }
    }
    (forward, depth)
}

enum Direction {
    Up(u32),
    Down(u32),
    Forward(u32),
}

fn parse_lines(lines: &[String]) -> Vec<Direction> {
    let mut instructions = Vec::new();
    for line in lines {
        let pieces: Vec<&str> = line.split(' ').collect();

        let instruction = match (pieces[0], pieces[1].parse::<u32>().unwrap()) {
            ("forward", x) => Direction::Forward(x),
            ("down", x) => Direction::Down(x),
            ("up", x) => Direction::Up(x),
            _ => panic!("OMG what did you give me?"),
        };

        instructions.push(instruction);
    }

    instructions
}

pub fn find_solution() -> Result<u32, Box<dyn std::error::Error>> {
    let lines = read_file("./src/day_2/input.txt".into())?;

    let instructions = parse_lines(&lines);

    let (horizontal_position, depth) = determine_direction(&instructions);

    Ok(horizontal_position * depth)
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(find_solution().unwrap(), 2_078_985_210);
    }

    #[test]
    fn playground() {
        let lines: Vec<String> = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        let instructions = parse_lines(&lines);

        let (horizontal_position, depth) = determine_direction(&instructions);

        assert_eq!(horizontal_position, 15);
        assert_eq!(depth, 60);
    }
}
