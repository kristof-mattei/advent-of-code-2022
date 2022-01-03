use std::cell::Cell;

use regex::Regex;

use crate::shared::{Day, PartSolution};

struct DeterministicDie {
    last: u32,
}

impl DeterministicDie {
    fn new() -> Self {
        Self { last: 0 }
    }

    fn roll(&mut self) -> u32 {
        self.last += 1;
        self.last
    }
}

struct QuantumDie {}

#[derive(PartialEq, Eq, Debug)]
struct Player {
    player_number: u32,
    position: Cell<u32>,
    score: Cell<u32>,
}

impl Player {
    fn new(player_number: u32, start_position: u32) -> Self {
        Self {
            player_number,
            position: Cell::new(start_position),
            score: 0.into(),
        }
    }

    fn moves(&self, how_much: u32) {
        self.position.set(self.position.get() + how_much);

        loop {
            let position = self.position.get();
            if position > 10 {
                self.position.set(position - 10);
            } else {
                break;
            }
        }

        self.score.set(self.score.get() + self.position.get());
    }
}

fn anybody_score_above(players: &[Player], above: u32) -> bool {
    players.iter().any(|p| p.score.get() >= above)
}

fn play(players: &[Player], until: u32) -> (u32, u32) {
    let mut die = DeterministicDie::new();

    let mut dice_rolls = 0;

    'outer: loop {
        for player in players {
            let mut rolled = Vec::new();

            for _ in 0..3 {
                rolled.push(die.roll());
                dice_rolls += 1;
            }

            player.moves(rolled.iter().sum());
            println!(
                "Player {} rolls {} and moves to place {} for a total score of {}.",
                player.player_number,
                rolled
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("+"),
                player.position.get(),
                player.score.get()
            );

            if anybody_score_above(players, until) {
                break 'outer;
            }
        }
    }

    (
        players.iter().map(|p| p.score.get()).min().unwrap(),
        dice_rolls,
    )
}

fn play_quantum(players: &[Player], until: u32) -> Vec<u64> {
    for player in players {
        for i in 1_u32..=3 {}
    }

    vec![]
}

fn parse_lines(lines: &[&str]) -> Vec<Player> {
    let mut players = Vec::new();

    let regex = Regex::new(r"Player (\d+) starting position: (\d+)").unwrap();

    for line in lines {
        let captures = regex.captures(line.trim()).unwrap();

        let player_number = captures[1].parse::<u32>().unwrap();
        let start_position = captures[2].parse::<u32>().unwrap();
        players.push(Player::new(player_number, start_position));
    }

    players
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let players = parse_lines(&lines);

        let result = play(&players, 1000);

        PartSolution::U32(result.0 * result.1)
    }

    fn part_2(&self) -> PartSolution {
        PartSolution::None
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt").lines().collect()
    }

    mod part_1 {

        use crate::{
            day_21::{parse_lines, play, Player, Solution},
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::U32(900_099));
        }

        #[test]
        fn example_parse_input() {
            let example_lines = get_example();

            let players = parse_lines(&example_lines);

            assert_eq!(vec![Player::new(1, 4), Player::new(2, 8)], players);
        }

        #[test]
        fn example() {
            let example_lines = get_example();

            let players = parse_lines(&example_lines);

            let result = play(&players, 1000);

            assert_eq!(739_785, result.0 * result.1);
        }
    }

    mod part_2 {
        use crate::{
            day_21::Solution,
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::None);
        }
    }
}
