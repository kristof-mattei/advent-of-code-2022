use std::cell::Cell;

use crate::shared::{Day, PartSolution};

struct Probe {
    velocity_x: Cell<i32>,
    velocity_y: Cell<i32>,
    x: Cell<i32>,
    y: Cell<i32>,
}

impl Probe {
    fn new(velocity_x: i32, velocity_y: i32) -> Self {
        Self {
            x: Cell::new(0),
            y: Cell::new(0),
            velocity_x: Cell::new(velocity_x),
            velocity_y: Cell::new(velocity_y),
        }
    }

    fn step(&self) {
        let velocity_x = self.velocity_x.get();
        let velocity_y = self.velocity_y.get();

        self.x.set(self.x.get() + velocity_x);
        self.y.set(self.y.get() + velocity_y);

        if velocity_x != 0 {
            self.velocity_x.set(velocity_x - 1);
        }

        self.velocity_y.set(velocity_y - 1);
    }
}

struct Target {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Target {
    fn probe_hit(&self, probe: &Probe) -> bool {
        let y_lower = self.y1.min(self.y2);
        let y_upper = self.y1.max(self.y2);

        probe.x.get() >= self.x1
            && probe.x.get() <= self.x2
            && probe.y.get() >= y_lower
            && probe.y.get() <= y_upper
    }
}

fn find_limit(limit: i32) -> i32 {
    let max = limit.abs();

    let mut current_max = 0;
    loop {
        if ((current_max * (current_max + 1)) / 2) > max {
            break;
        }

        current_max += 1;
    }

    current_max
}

enum Hit {
    Hit(i32),
    Missed,
}

fn launch_probe(velocity_x: i32, velocity_y: i32, target: &Target) -> Hit {
    let probe = Probe::new(velocity_x, velocity_y);

    let mut max = i32::MIN;

    loop {
        probe.step();

        let y = probe.y.get();

        if y > max {
            max = y;
        }

        if target.probe_hit(&probe) {
            return Hit::Hit(max);
        }

        if probe.y.get() < target.y1 {
            return Hit::Missed;
        }
    }
}

fn find_max_y(target: &Target) -> i32 {
    let mut max_y = i32::MIN;

    let min_x = find_limit(target.x1);
    let max_x = find_limit(target.x2);

    let min_y = find_limit(target.y2);

    let x_range = min_x..=max_x;

    println!("Launching X between {:?}", x_range);

    for launch_x in x_range {
        let mut max_y_for_x = i32::MIN;

        let y_range = min_y..=target.y1.abs();

        for launch_y in y_range {
            let result = launch_probe(launch_x, launch_y, target);
            match result {
                Hit::Hit(y) => {
                    if y > max_y_for_x {
                        max_y_for_x = y;
                        println!("MAX HIT AT {},{}", launch_x, launch_y);
                    }
                },
                Hit::Missed => {},
            }
        }

        if max_y_for_x > max_y {
            max_y = max_y_for_x;
        }
    }

    max_y
}

fn count_hits(target: &Target) -> u32 {
    let mut hits = 0;

    let limit_x = find_limit(target.x1);
    let limit_y = target.y1.abs();

    let x_range = limit_x..=target.x2;

    for launch_x in x_range {
        let y_range = -limit_y..=limit_y;

        for launch_y in y_range {
            let result = launch_probe(launch_x, launch_y, target);
            match result {
                Hit::Hit(_) => {
                    hits += 1;
                    println!("HIT AT {},{}", launch_x, launch_y);
                },
                Hit::Missed => {},
            }
        }
    }

    hits
}

fn parse_lines(_lines: &[String]) -> Target {
    // let line = lines[0].replace("target area: ", "");

    Target {
        x1: 88,
        x2: 125,
        y1: -157,
        y2: -103,
    }
}
pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let target = parse_lines(&lines);

        let max = find_max_y(&target);

        PartSolution::I32(max)
    }

    fn part_2(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        let target = parse_lines(&lines);

        let hits = count_hits(&target);

        PartSolution::U32(hits)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use std::cell::Cell;

        use crate::{
            day_17::{find_max_y, Probe, Solution, Target},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::I32(12246));
        }

        #[test]
        fn example() {
            // ...

            // let parsed = parse_input();
            let target = Target {
                x1: 20,
                x2: 30,
                y1: -10,
                y2: -5,
            };

            let probe = Probe::new(7, 2);
            assert_eq!(0, probe.x.get());
            assert_eq!(0, probe.y.get());

            assert_eq!(7, probe.velocity_x.get());
            assert_eq!(2, probe.velocity_y.get());

            probe.step();

            assert_eq!(7, probe.x.get());
            assert_eq!(2, probe.y.get());

            assert_eq!(6, probe.velocity_x.get());
            assert_eq!(1, probe.velocity_y.get());

            probe.step();

            assert_eq!(13, probe.x.get());
            assert_eq!(3, probe.y.get());

            assert_eq!(5, probe.velocity_x.get());
            assert_eq!(0, probe.velocity_y.get());

            probe.step();

            assert_eq!(18, probe.x.get());
            assert_eq!(3, probe.y.get());

            assert_eq!(4, probe.velocity_x.get());
            assert_eq!(-1, probe.velocity_y.get());

            probe.step();

            assert_eq!(22, probe.x.get());
            assert_eq!(2, probe.y.get());

            assert_eq!(3, probe.velocity_x.get());
            assert_eq!(-2, probe.velocity_y.get());

            probe.step();

            assert_eq!(25, probe.x.get());
            assert_eq!(0, probe.y.get());

            assert_eq!(2, probe.velocity_x.get());
            assert_eq!(-3, probe.velocity_y.get());

            probe.step();

            assert_eq!(27, probe.x.get());
            assert_eq!(-3, probe.y.get());

            assert_eq!(1, probe.velocity_x.get());
            assert_eq!(-4, probe.velocity_y.get());

            probe.step();

            assert_eq!(28, probe.x.get());
            assert_eq!(-7, probe.y.get());

            assert_eq!(0, probe.velocity_x.get());
            assert_eq!(-5, probe.velocity_y.get());

            assert!(target.probe_hit(&probe));
        }

        #[test]
        fn example_2() {
            // ...

            // let parsed = parse_input();
            let target = Target {
                x1: 20,
                x2: 30,
                y1: -10,
                y2: -5,
            };

            let probe = Probe::new(6, 3);

            for _ in 0..9 {
                probe.step();
            }

            assert!(target.probe_hit(&probe));
        }

        #[test]
        fn example_3() {
            // ...

            // let parsed = parse_input();
            let target = Target {
                x1: 20,
                x2: 30,
                y1: -10,
                y2: -5,
            };

            let probe = Probe::new(9, 0);

            for _ in 0..4 {
                probe.step();
            }

            assert!(target.probe_hit(&probe));
        }

        #[test]
        fn example_max() {
            // ...

            // let parsed = parse_input();
            let target = Target {
                x1: 20,
                x2: 30,
                y1: -10,
                y2: -5,
            };

            let probe = Probe::new(6, 9);

            let mut max: i32 = i32::MIN;
            while !target.probe_hit(&probe) {
                probe.step();
                let y = probe.y.get();
                if y > max {
                    max = y;
                }
            }

            assert_eq!(45, max);
        }

        #[test]
        fn example_find_max() {
            // ...

            // let parsed = parse_input();
            let target = Target {
                x1: 20,
                x2: 30,
                y1: -10,
                y2: -5,
            };

            let max = find_max_y(&target);

            assert_eq!(45, max);
        }

        #[test]
        fn test_hit_positive_y() {
            let probe = Probe {
                x: Cell::new(5),
                y: Cell::new(10),
                velocity_x: Cell::new(0),
                velocity_y: Cell::new(0),
            };

            let target = Target {
                x1: 4,
                x2: 6,
                y1: 5,
                y2: 15,
            };

            assert!(target.probe_hit(&probe));
        }

        #[test]
        fn test_hit_positive_negative_y() {
            let probe = Probe {
                x: Cell::new(5),
                y: Cell::new(0),
                velocity_x: Cell::new(0),
                velocity_y: Cell::new(0),
            };

            let target = Target {
                x1: 4,
                x2: 6,
                y1: 10,
                y2: -10,
            };

            assert!(target.probe_hit(&probe));
        }

        #[test]
        fn test_hit_negative_y() {
            let probe = Probe {
                x: Cell::new(5),
                y: Cell::new(-10),
                velocity_x: Cell::new(0),
                velocity_y: Cell::new(0),
            };

            let target = Target {
                x1: 4,
                x2: 6,
                y1: -5,
                y2: -15,
            };

            assert!(target.probe_hit(&probe));
        }
    }

    mod part_2 {

        use crate::{
            day_17::{count_hits, Solution, Target},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::U32(3528));
        }

        #[test]
        fn example_count_hits() {
            // ...

            // let parsed = parse_input();
            let target = Target {
                x1: 20,
                x2: 30,
                y1: -10,
                y2: -5,
            };

            let hits = count_hits(&target);

            assert_eq!(112, hits);
        }
    }
}
