use std::collections::BTreeSet;

use advent_of_code_2022::shared::{PartSolution, Parts};
use hashbrown::{hash_map::Entry, HashMap};
use regex::{Captures, Regex};

advent_of_code_2022::solution!(4_883_971, 12_691_026_767_556usize);

#[derive(Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Type {
    Sensor,
    Beacon,
    Impossible,
}

fn get_coordinate(captures: &Captures, name: &str) -> isize {
    captures
        .name(name)
        .and_then(|v| v.as_str().parse().ok())
        .unwrap()
}

fn parse_lines(lines: &str) -> Vec<(Coordinate, Coordinate)> {
    let regex = Regex::new( r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap();

    let mut sensors_and_beacons = Vec::new();

    for sensor_line in regex.captures_iter(lines) {
        let sensor_x: isize = get_coordinate(&sensor_line, "sensor_x");
        let sensor_y: isize = get_coordinate(&sensor_line, "sensor_y");
        let beacon_x: isize = get_coordinate(&sensor_line, "beacon_x");
        let beacon_y: isize = get_coordinate(&sensor_line, "beacon_y");

        sensors_and_beacons.push((
            Coordinate {
                x: sensor_x,
                y: sensor_y,
            },
            Coordinate {
                x: beacon_x,
                y: beacon_y,
            },
        ));
    }

    sensors_and_beacons
}

fn find_taken_cells(
    sensors_and_beacons: &[(Coordinate, Coordinate)],
    line: isize,
    min: isize,
    max: isize,
) -> HashMap<isize, Type> {
    let mut taken_cells = HashMap::<_, Type>::new();

    for (sensor, beacon) in sensors_and_beacons {
        if sensor.y == line {
            taken_cells.insert(sensor.x, Type::Sensor);
        }

        if beacon.y == line {
            taken_cells.insert(beacon.x, Type::Beacon);
        }
    }

    for (sensor, beacon) in sensors_and_beacons {
        let manhattan = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();

        let diff: isize = isize::try_from(sensor.y.abs_diff(line)).unwrap();

        if diff <= manhattan {
            for x in isize::max(min, sensor.x - (manhattan - diff))
                ..=isize::min(sensor.x + (manhattan - diff), max)
            {
                match taken_cells.entry(x) {
                    Entry::Occupied(_) => {},
                    Entry::Vacant(v) => {
                        v.insert(Type::Impossible);
                    },
                }
            }
        }
    }

    taken_cells
}

fn find_empty(
    sensors_and_beacons: &[(Coordinate, Coordinate)],
    line: isize,
    max: isize,
) -> BTreeSet<(isize, isize)> {
    let mut ranges = BTreeSet::new();

    for (sensor, beacon) in sensors_and_beacons {
        if sensor.y == line && sensor.x > 0 {
            ranges.insert((sensor.x, sensor.x + 1));
        }

        if beacon.y == line && beacon.x > 0 {
            ranges.insert((beacon.x, beacon.x + 1));
        }
    }

    for (sensor, beacon) in sensors_and_beacons {
        let manhattan = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();

        let diff: isize = isize::try_from(sensor.y.abs_diff(line)).unwrap();

        let start = isize::max(0, sensor.x - (manhattan - diff));
        let end = isize::min(sensor.x + (manhattan - diff) + 1, max);

        if diff <= manhattan && (0..=max).contains(&start) || (0..=max).contains(&end) {
            ranges.insert((start, end));
        }
    }

    ranges
}

fn find_impossible_spots_on_line(
    sensors_and_beacons: &[(Coordinate, Coordinate)],
    line: isize,
) -> usize {
    let taken_cells = find_taken_cells(sensors_and_beacons, line, isize::MIN, isize::MAX);

    taken_cells
        .iter()
        .filter(|(_, t)| t == &&Type::Impossible)
        .count()
}

fn find_only_possible_spot(
    sensors_and_beacons: &[(Coordinate, Coordinate)],
    max_included: isize,
) -> usize {
    let mut last_percent = 101;

    for line in 0..=max_included {
        let percent = (line * 100) / max_included;
        if last_percent != percent {
            last_percent = percent;

            eprintln!("Line {} of {}, {}%", line, max_included, last_percent);
        }

        let mut taken_cells = find_empty(sensors_and_beacons, line, max_included);

        // check for 0
        if taken_cells.first().map_or(false, |&(l, _)| l != 0) {
            return line.try_into().unwrap();
        }

        let (mut total_l, mut total_u) = taken_cells.pop_first().unwrap();

        for (l, u) in taken_cells {
            let left_contained = (total_l..=total_u).contains(&l);
            let right_contained = (total_l..=total_u).contains(&u);

            if left_contained && right_contained {
            } else if left_contained || right_contained {
                total_l = isize::min(total_l, l);
                total_u = isize::max(total_u, u);
            } else {
                return ((total_u.unsigned_abs()) * 4_000_000) + line.unsigned_abs();
            }
        }
    }

    panic!()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let taken_on_2_000_000 = find_impossible_spots_on_line(&parsed, 2_000_000);

        taken_on_2_000_000.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_lines(input);

        let only_possible_spot = find_only_possible_spot(&parsed, 4_000_000);

        only_possible_spot.into()
    }
}

#[cfg(test)]
mod tests {

    mod part_1 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts};

        use crate::{find_impossible_spots_on_line, parse_lines, Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(4_883_971),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            let parsed = parse_lines(&read_file("examples", &DAY));

            assert_eq!(
                PartSolution::USize(26),
                find_impossible_spots_on_line(&parsed, 10).into()
            );
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts};

        use crate::{find_only_possible_spot, parse_lines, Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(12_691_026_767_556),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            let parsed = parse_lines(&read_file("examples", &DAY));

            assert_eq!(
                PartSolution::USize(56_000_011),
                find_only_possible_spot(&parsed, 20).into()
            );
        }
    }
}
