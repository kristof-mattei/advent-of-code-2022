use std::collections::HashMap;

use regex::{Captures, Regex};

use crate::shared::{Day, PartSolution};

fn get_value<T: std::str::FromStr>(captures: &Captures, name: &str) -> T {
    captures
        .name(name)
        .and_then(|v| v.as_str().parse().ok())
        .unwrap()
}

fn parse_lines(lines: &str) -> HashMap<String, ValveData> {
    let mut mapping = HashMap::new();

    let regex = Regex::new(
        r"Valve (?P<valve>\w\w) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<tunnels>(?:\w\w(?:, )?)*)",
    ).unwrap();

    for captures in regex.captures_iter(lines) {
        let valve: String = get_value::<String>(&captures, "valve");
        let flow_rate: usize = get_value(&captures, "flow_rate");
        let to: Vec<String> = get_value::<String>(&captures, "tunnels")
            .split(", ")
            .map(Into::into)
            .collect::<Vec<_>>();

        mapping.insert(valve, ValveData { flow_rate, to });
    }

    mapping
}

struct ValveData {
    flow_rate: usize,
    to: Vec<String>,
}

fn is_visited(bit_representation_a: usize, bit_representation_b: usize) -> bool {
    (bit_representation_a & bit_representation_b) > 0
}

struct Solver {
    valve_mapping: HashMap<String, ValveData>,
    valves_having_positive_rate: Vec<String>,
    valve_bit_repr: HashMap<String, usize>,
    min_time: HashMap<String, HashMap<String, usize>>,
}

impl Solver {
    fn build(valve_mapping: HashMap<String, ValveData>) -> Self {
        let mut valves_having_positive_rate = valve_mapping
            .iter()
            .filter_map(|(valve_name, &ValveData { flow_rate, .. })| {
                (flow_rate > 0).then(|| valve_name.clone())
            })
            .collect::<Vec<_>>();

        valves_having_positive_rate.sort();

        let valve_bit_repr = valves_having_positive_rate
            .iter()
            .enumerate()
            .map(|(i, v)| (v.clone(), 1 << i))
            .collect::<HashMap<_, _>>();

        let mut min_time = valve_mapping
            .keys()
            .map(|valve_from| {
                let inner = valve_mapping
                    .keys()
                    .map(|valve_to| {
                        (
                            valve_to.clone(),
                            if valve_mapping[valve_from].to.contains(valve_to) {
                                1
                            } else {
                                usize::MAX
                            },
                        )
                    })
                    .collect::<HashMap<String, _>>();

                (valve_from.clone(), inner)
            })
            .collect::<HashMap<_, _>>();

        for mid in valve_mapping.keys() {
            for start in valve_mapping.keys() {
                for end in valve_mapping.keys() {
                    let new_min = usize::min(
                        min_time[start][end],
                        min_time[start][mid].saturating_add(min_time[mid][end]),
                    );

                    let start_min = min_time.get_mut(start).unwrap();
                    let end_min = start_min.get_mut(end).unwrap();
                    *end_min = new_min;
                }
            }
        }

        Self {
            valve_mapping,
            valves_having_positive_rate,
            valve_bit_repr,
            min_time,
        }
    }

    fn solve_recursive(
        &self,
        minutes_left: usize,
        current_valve: &str,
        opened_bit_representation: usize,
        rate: usize,
        max_rate_map_bit_representation: &mut HashMap<usize, usize>,
    ) {
        let new_max = usize::max(
            max_rate_map_bit_representation
                .get(&opened_bit_representation)
                .map_or(0, |m| *m),
            rate,
        );

        let mr = max_rate_map_bit_representation
            .entry(opened_bit_representation)
            .or_default();

        *mr = new_max;

        for valve_name in &self.valves_having_positive_rate {
            let new_minutes_left =
                minutes_left.saturating_sub(self.min_time[current_valve][valve_name] + 1);
            let new_rate = rate + new_minutes_left * self.valve_mapping[valve_name].flow_rate;
            let new_opened_bit_representation =
                opened_bit_representation | self.valve_bit_repr[valve_name];

            if new_minutes_left == 0
                || is_visited(self.valve_bit_repr[valve_name], opened_bit_representation)
            {
                continue;
            }

            self.solve_recursive(
                new_minutes_left,
                valve_name,
                new_opened_bit_representation,
                new_rate,
                max_rate_map_bit_representation,
            );
        }
    }
}

fn release_pressure(valve_mapping: HashMap<String, ValveData>) -> usize {
    let solver = Solver::build(valve_mapping);

    let mut max_rate_map_bit_representation = HashMap::new();

    solver.solve_recursive(30, "AA", 0, 0, &mut max_rate_map_bit_representation);

    max_rate_map_bit_representation
        .into_values()
        .max()
        .unwrap_or(0)
}

fn release_pressure_with_elephant(valve_mapping: HashMap<String, ValveData>) -> usize {
    let solver = Solver::build(valve_mapping);

    let mut max_rate_map_bit_representation = HashMap::new();

    solver.solve_recursive(
        26,
        &String::from("AA"),
        0,
        0,
        &mut max_rate_map_bit_representation,
    );

    let mut result = usize::MIN;

    for (human, human_max_rate) in &max_rate_map_bit_representation {
        for (elephant, elephant_max_rate) in &max_rate_map_bit_representation {
            if (*human & *elephant) == 0 {
                result = usize::max(result, human_max_rate + elephant_max_rate);
            }
        }
    }

    result
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: &str = include_str!("input.txt");

        let valve_mapping = parse_lines(lines);

        let pressure = release_pressure(valve_mapping);

        pressure.into()
    }

    fn part_2(&self) -> PartSolution {
        let lines: &str = include_str!("input.txt");

        let valve_mapping = parse_lines(lines);

        let pressure_with_elephant = release_pressure_with_elephant(valve_mapping);

        pressure_with_elephant.into()
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> &'static str {
        include_str!("example.txt")
    }

    mod part_1 {
        use super::super::{parse_lines, Solution};
        use super::get_example;
        use crate::day_16::release_pressure;
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(1820), (Solution {}).part_1());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let valves_and_tunnels = parse_lines(lines);

            let pressure = release_pressure(valves_and_tunnels);

            assert_eq!(1651, pressure);
        }
    }

    mod part_2 {
        use super::super::{parse_lines, Solution};
        use super::get_example;
        use crate::day_16::release_pressure_with_elephant;
        use crate::shared::{Day, PartSolution};

        #[test]
        fn outcome() {
            assert_eq!(PartSolution::USize(2602), (Solution {}).part_2());
        }

        #[test]
        fn example() {
            let lines = get_example();

            let valve_mapping = parse_lines(lines);

            let elephant = release_pressure_with_elephant(valve_mapping);

            assert_eq!(1707, elephant);
        }
    }
}
