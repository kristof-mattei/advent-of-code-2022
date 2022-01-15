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

impl Game {
    fn new(players: Vec<Player>) -> Self {
        Self {
            players,
            current_player: 0,
        }
    }
}

fn get_quantum_die_rolls() -> HashMap<u32, u32> {
    let mut rolls = Vec::new();
    for r1 in 1..=3 {
        for r2 in 1..=3 {
            for r3 in 1..=3 {
                rolls.push(vec![r1, r2, r3]);
            }
        }
    }

    let mut rolls_sum_with_count = HashMap::new();

    for roll in rolls {
        rolls_sum_with_count
            .entry(roll.iter().sum::<u32>())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    rolls_sum_with_count
}

fn play_quantum(cache: &mut HashMap<Game, Vec<u64>>, game: &Game, until: u32) -> Vec<u64> {
    let rolls = get_quantum_die_rolls();

    let mut results = Vec::new();
    results.resize(game.players.len(), 0);

    let next_player = (game.current_player + 1) % game.players.len();

    for (roll_sum, occurence) in &rolls {
        let mut new_game = game.clone();
        new_game.players[game.current_player].r#move(*roll_sum);

        if new_game.players[game.current_player].score >= until {
            results[game.current_player] += u64::from(*occurence);
        } else {
            new_game.current_player = next_player;

            // the borrow checker doesn't like this
            // let cached = cache.entry(new_game).or_insert_with_key(|ng| play_quantum(cache, ng, until));

            // do we have the value cached?
            let cached = cache.contains_key(&new_game);

            // calculate the value to insert
            let to_insert = if cached {
                None
            } else {
                Some(play_quantum(cache, &new_game, until))
            };

            let new_game_results = match to_insert {
                None => &cache[&new_game],
                Some(r) => cache.entry(new_game).or_insert(r),
            };

            new_game_results
                .iter()
                .enumerate()
                .for_each(|(i, c)| results[i] += c * u64::from(*occurence));
        }
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

        let result = play_quantum(&mut HashMap::new(), &Game::new(players), 21);

        PartSolution::U64(*result.iter().max().unwrap())
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
        use std::collections::HashMap;

        use crate::{
            day_21::{parse_lines, play_quantum, test::get_example, Game, Solution},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!(
                (Solution {}).part_2(),
                PartSolution::U64(306_719_685_234_774)
            );
        }

        #[test]
        fn example() {
            let example_lines = get_example();

            let players = parse_lines(&example_lines);

            let result = play_quantum(&mut HashMap::new(), &Game::new(players), 21);

            println!("{:?}", result);
            assert_eq!(444_356_092_776_315, result[0]);
            assert_eq!(341_960_390_180_808, result[1]);
        }
    }
}
