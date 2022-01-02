use crate::shared::{Day, PartSolution};

#[derive(Debug)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

struct Permutation {
    beacons: Vec<Beacon>,
}

struct Scanner {
    permutations: Vec<Permutation>,
}

fn calculate_permutations(scanners_with_single_permutation: Vec<Permutation>) -> Vec<Scanner> {
    let mut s: Vec<Scanner> = Vec::new();

    let permutation_fns = [
        // |b: &Beacon| Beacon {
        //     x: b.x,
        //     y: b.y,
        //     z: b.z,
        // },
        |b: &Beacon| Beacon {
            x: b.x,
            y: -b.z,
            z: b.y,
        },
        |b: &Beacon| Beacon {
            x: b.x,
            y: -b.y,
            z: -b.z,
        },
        |b: &Beacon| Beacon {
            x: b.x,
            y: b.z,
            z: -b.y,
        },
        |b: &Beacon| Beacon {
            x: -b.y,
            y: b.x,
            z: b.z,
        },
        |b: &Beacon| Beacon {
            x: b.z,
            y: b.x,
            z: b.y,
        },
        |b: &Beacon| Beacon {
            x: b.y,
            y: b.x,
            z: -b.z,
        },
        |b: &Beacon| Beacon {
            x: -b.z,
            y: b.x,
            z: -b.y,
        },
        |b: &Beacon| Beacon {
            x: -b.x,
            y: -b.y,
            z: b.z,
        },
        |b: &Beacon| Beacon {
            x: -b.x,
            y: -b.z,
            z: -b.y,
        },
        |b: &Beacon| Beacon {
            x: -b.x,
            y: b.y,
            z: -b.z,
        },
        |b: &Beacon| Beacon {
            x: -b.x,
            y: b.z,
            z: b.y,
        },
        |b: &Beacon| Beacon {
            x: b.y,
            y: -b.x,
            z: b.z,
        },
        |b: &Beacon| Beacon {
            x: b.z,
            y: -b.x,
            z: -b.y,
        },
        |b: &Beacon| Beacon {
            x: -b.y,
            y: -b.x,
            z: -b.z,
        },
        |b: &Beacon| Beacon {
            x: -b.z,
            y: -b.x,
            z: b.y,
        },
        |b: &Beacon| Beacon {
            x: -b.z,
            y: b.y,
            z: b.x,
        },
        |b: &Beacon| Beacon {
            x: b.y,
            y: b.z,
            z: b.x,
        },
        |b: &Beacon| Beacon {
            x: b.z,
            y: -b.y,
            z: b.x,
        },
        |b: &Beacon| Beacon {
            x: -b.y,
            y: -b.z,
            z: b.x,
        },
        |b: &Beacon| Beacon {
            x: -b.z,
            y: -b.y,
            z: -b.x,
        },
        |b: &Beacon| Beacon {
            x: -b.y,
            y: b.z,
            z: -b.x,
        },
        |b: &Beacon| Beacon {
            x: b.z,
            y: b.y,
            z: -b.x,
        },
        |b: &Beacon| Beacon {
            x: b.y,
            y: -b.z,
            z: -b.x,
        },
    ];
    for original_permutation in scanners_with_single_permutation {
        let mut permutations = Vec::new();

        for permutation_fn in permutation_fns {
            let mut permutation = Vec::new();

            for beacon in &original_permutation.beacons {
                permutation.push(permutation_fn(beacon));
            }

            permutations.push(Permutation {
                beacons: permutation,
            });
        }

        permutations.push(original_permutation);

        s.push(Scanner { permutations });
    }

    s
}

fn parse_beacon_line(line: &str) -> Beacon {
    let split: Vec<&str> = line.split(',').collect();
    let x = split[0].parse::<i32>().unwrap();
    let y = split[1].parse::<i32>().unwrap();
    let z = split[2].parse::<i32>().unwrap();

    Beacon { x, y, z }
}

fn parse_lines(lines: &[&str]) -> Vec<Permutation> {
    let raw_scanners = lines.split(|l| (*l).is_empty());

    let mut scanners = Vec::new();

    for lines in raw_scanners {
        let mut beacons = Vec::new();

        for line in lines {
            if !line.starts_with("---") {
                let beacon = parse_beacon_line(line);
                println!("Beacon: {:?}", beacon);
                beacons.push(beacon);
            }
        }

        scanners.push(Permutation { beacons });
    }

    scanners
}
pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let _scanners = parse_lines(&lines);

        PartSolution::None
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
            day_19::{calculate_permutations, parse_lines, Solution},
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::None);
        }

        #[test]
        fn example() {
            let example_lines = get_example();

            let scanners = parse_lines(&example_lines);

            let scanners_with_permutations = calculate_permutations(scanners);
        }
    }

    mod part_2 {

        use crate::{
            day_19::Solution,
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::None);
        }
    }
}
