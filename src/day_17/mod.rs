use std::cell::Cell;

use crate::shared::{Day, PartSolution};

struct Probe {
    velocity_x: Cell<u32>,
    velocity_y: Cell<i32>,
    x: Cell<u32>,
    y: Cell<i32>,
}

impl Probe {
    fn new(velocity_x: u32, velocity_y: i32) -> Self {
        Self {
            x: Cell::new(0),
            y: Cell::new(0),
            velocity_x: Cell::new(velocity_x),
            velocity_y: Cell::new(velocity_y),
        }
    }
}

struct Target {
    x1: u32,
    x2: u32,
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

fn step(probe: &Probe) {
    let velocity_x = probe.velocity_x.get();
    let velocity_y = probe.velocity_y.get();

    probe.x.set(probe.x.get() + velocity_x);
    probe.y.set(probe.y.get() + velocity_y);

    if velocity_x != 0 {
        probe.velocity_x.set(velocity_x - 1);
    }

    probe.velocity_y.set(velocity_y - 1);
}

fn max_launch(start_at: u32, upper: u32) -> u32 {
    if upper == 0 {
        return 0;
    }

    let mut max = start_at;
    loop {
        if ((max * max) / 2) + (max / 2) > upper {
            break;
        }

        max += 1;
    }

    max
}

enum Hit {
    Missed(u32),
    Hit(i32),
}

fn get_max_y(velocity_x: u32, velocity_y: i32, target: &Target) -> Hit {
    let probe = Probe::new(velocity_x, velocity_y);

    let mut max = i32::MIN;

    loop {
        step(&probe);

        let y = probe.y.get();

        if y > max {
            max = y;
        }

        if target.probe_hit(&probe) {
            return Hit::Hit(max);
        }

        if probe.x.get() > target.x2 || probe.y.get() < i32::min(target.y1, target.y2) {
            // we missed if we flew past (as we can never go back)
            // or landed below

            return Hit::Missed(probe.x.get());
        }
    }
}

fn find_max_y_inner(launch_x: u32, target: &Target) -> i32 {
    let mut max_y_for_x = i32::MIN;

    let mut launch_y = 0;

    let mut last_max_x = u32::MIN;

    loop {
        println!("Launching at ({},{})", launch_x, launch_y);
        let result = get_max_y(launch_x, launch_y, target);
        match result {
            Hit::Hit(y) => {
                println!("Hit! Y was highest at {}", y);
                if y > max_y_for_x {
                    max_y_for_x = y;
                }
            }
            Hit::Missed(x) => {
                // see if we're getting further
                if x > last_max_x {
                    println!(
                        "Missed, last time landed at {}, now at {}, so we're trying again",
                        last_max_x, x
                    );

                    last_max_x = x;
                } else {
                    println!("Missed, landed at the same x as last time ({})", x);

                    // there is a bug here I think were we should have a different way of detecting again

                    break;
                }
            }
        }

        launch_y += 1;
    }

    max_y_for_x
}

fn find_max_y(target: &Target) -> i32 {
    let min_x: u32 = max_launch(0, target.x1);
    let max_x: u32 = max_launch(min_x, target.x2);

    let mut max_y = i32::MIN;

    println!("Launching X between {}..={}", min_x, max_x);

    for launch_x in min_x..=max_x {
        let max_y_for_x = find_max_y_inner(launch_x, target);

        if max_y_for_x > max_y {
            max_y = max_y_for_x;
        }
    }

    max_y
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
        PartSolution::None
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use std::cell::Cell;

        use crate::{
            day_17::{find_max_y, step, Probe, Solution, Target},
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::None);
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

            step(&probe);

            assert_eq!(7, probe.x.get());
            assert_eq!(2, probe.y.get());

            assert_eq!(6, probe.velocity_x.get());
            assert_eq!(1, probe.velocity_y.get());

            step(&probe);

            assert_eq!(13, probe.x.get());
            assert_eq!(3, probe.y.get());

            assert_eq!(5, probe.velocity_x.get());
            assert_eq!(0, probe.velocity_y.get());

            step(&probe);

            assert_eq!(18, probe.x.get());
            assert_eq!(3, probe.y.get());

            assert_eq!(4, probe.velocity_x.get());
            assert_eq!(-1, probe.velocity_y.get());

            step(&probe);

            assert_eq!(22, probe.x.get());
            assert_eq!(2, probe.y.get());

            assert_eq!(3, probe.velocity_x.get());
            assert_eq!(-2, probe.velocity_y.get());

            step(&probe);

            assert_eq!(25, probe.x.get());
            assert_eq!(0, probe.y.get());

            assert_eq!(2, probe.velocity_x.get());
            assert_eq!(-3, probe.velocity_y.get());

            step(&probe);

            assert_eq!(27, probe.x.get());
            assert_eq!(-3, probe.y.get());

            assert_eq!(1, probe.velocity_x.get());
            assert_eq!(-4, probe.velocity_y.get());

            step(&probe);

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
                step(&probe);
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
                step(&probe);
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
                step(&probe);
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

    mod part_2 {}
}
