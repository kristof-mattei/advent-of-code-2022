type Playfield = [[u32; 5]; 5];

fn calculate_winnings(_drawings: &[u32], _play_fields: &[Playfield]) -> (u32, u32) {
    (0, 0)
}

fn parse_lines(lines: &[String]) -> (Vec<u32>, Vec<Playfield>) {
    let drawings = lines[0]
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    for chunk in lines
        .to_vec()
        .iter()
        .skip(2)
        .collect::<Vec<&String>>()
        .chunks(5)
    {
        println!("{:?}", chunk);
    }

    let playfield1: Playfield = [
        [0, 1, 2, 3, 4],
        [0, 1, 2, 3, 4],
        [0, 1, 2, 3, 4],
        [0, 1, 2, 3, 4],
        [0, 1, 2, 3, 4],
    ];

    (drawings, vec![playfield1])
}

pub fn find_solution() -> u32 {
    let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

    let (drawings, play_fields) = parse_lines(&lines);

    let (gamma, epsilon) = calculate_winnings(&drawings, &play_fields);

    gamma * epsilon
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn outcome() {
        assert_eq!(find_solution(), 0);
    }

    #[test]
    fn playground() {
        let lines: Vec<String> = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        let (drawings, play_fields) = parse_lines(&lines);

        let (_, _) = calculate_winnings(&drawings, &play_fields);
    }
}
