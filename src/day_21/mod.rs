use std::collections::HashMap;

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

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
struct Player {
    player_number: u32,
    position: u32,
    score: u32,
}

impl Player {
    fn new(player_number: u32, start_position: u32) -> Self {
        Self {
            player_number,
            position: start_position,
            score: 0,
        }
    }

    fn r#move(&mut self, how_much: u32) {
        self.position += how_much;

        loop {
            let position = self.position;
            if position > 10 {
                self.position = position - 10;
            } else {
                break;
            }
        }

        self.score += self.position;
    }
}

fn play(mut players: Vec<Player>, until: u32) -> (u32, u32) {
    let mut die = DeterministicDie::new();

    let mut dice_rolls = 0;

    'outer: loop {
        for player in &mut players {
            let mut rolled = Vec::new();

            for _ in 0..3 {
                rolled.push(die.roll());
                dice_rolls += 1;
            }

            player.r#move(rolled.iter().sum());
            println!(
                "Player {} rolls {} and moves to place {} for a total score of {}.",
                player.player_number,
                rolled
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("+"),
                player.position,
                player.score
            );

            if player.score >= until {
                break 'outer;
            }
        }
    }

    (players.iter().map(|p| p.score).min().unwrap(), dice_rolls)
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

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Game {
    players: Vec<Player>,
    current_player: usize,
}

fn play_quantum(players: Vec<Player>, until: u32) -> Vec<u128> {
    let mut results = Vec::new();
    results.resize(2, 0);

    let mut games: HashMap<Game, u128> = HashMap::from([(
        Game {
            players,
            current_player: 0,
        },
        1,
    )]);

    loop {
        // get first value where count > 0
        // and remove it
        // check if it's a win
        // if so, add wins
        // if not, duplicatae each entry with a count > 0
        let mut new_games = HashMap::new();
        if let Some((game, count)) = games.iter_mut().find(|(_, c)| **c > 0) {
            let next_player = (game.current_player + 1) % game.players.len();

            let c = *count;

            for r in 1..=3 {
                let mut new_game = game.clone();
                new_game.players[new_game.current_player].r#move(r);

                if new_game.players[new_game.current_player].score >= until {
                    results[new_game.current_player] += c;
                } else {
                    new_game.current_player = next_player;
                    new_games.insert(new_game, c);
                }
            }

            *count = 0;
        } else {
            break;
        }

        games
            .iter_mut()
            .filter(|(_, c)| **c > 0)
            .for_each(|(_, c)| *c += new_games.values().sum::<u128>());

        for (new_game, count) in new_games {
            games
                .entry(new_game)
                .and_modify(|c| *c += count)
                .or_insert(count);
        }

        //println!("{} games to play", games.values().sum::<u64>());
    }
    results
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let players = parse_lines(&lines);

        let result = play(players, 1000);

        PartSolution::U32(result.0 * result.1)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let players = parse_lines(&lines);

        let result = play_quantum(players, 21);

        PartSolution::U128(*result.iter().max().unwrap())
        // PartSolution::U32(result.0 * result.1)
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

            let result = play(players, 1000);

            assert_eq!(739_785, result.0 * result.1);
        }
    }

    mod part_2 {
        use crate::{
            day_21::{parse_lines, play_quantum, test::get_example, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::None);
        }

        #[test]
        fn example() {
            let example_lines = get_example();

            let players = parse_lines(&example_lines);

            let result = play_quantum(players, 21);

            println!("{:?}", result);
            assert_eq!(444_356_092_776_315, result[0]);
            assert_eq!(341_960_390_180_808, result[1]);
        }
    }
}
