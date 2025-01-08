use advent_of_code_2022::shared::{PartSolution, Parts};
use hashbrown::HashSet;

advent_of_code_2022::solution!(3662, 2060);

#[derive(PartialEq, Eq, Hash, Clone)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cube {{ {}, {}, {} }}", self.x, self.y, self.z)
    }
}

impl From<(isize, isize, isize)> for Cube {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Cube { x, y, z }
    }
}

impl Cube {
    fn neighbors(&self) -> [Cube; 6] {
        [
            (self.x + 1, self.y, self.z).into(),
            (self.x - 1, self.y, self.z).into(),
            (self.x, self.y + 1, self.z).into(),
            (self.x, self.y - 1, self.z).into(),
            (self.x, self.y, self.z + 1).into(),
            (self.x, self.y, self.z - 1).into(),
        ]
    }

    fn in_bounds(&self, min: &Cube, max: &Cube) -> bool {
        ((min.x - 1)..=(max.x + 1)).contains(&self.x)
            && ((min.y - 1)..=(max.y + 1)).contains(&self.y)
            && ((min.z - 1)..=(max.z + 1)).contains(&self.z)
    }
}

fn parse_input(input: &str) -> HashSet<Cube> {
    input
        .trim()
        .lines()
        .map(|line| {
            let pieces = line
                .trim()
                .split(',')
                .map(|v| v.parse::<isize>().unwrap())
                .collect::<Vec<_>>();

            Cube {
                x: pieces[0],
                y: pieces[1],
                z: pieces[2],
            }
        })
        .collect()
}

fn count_surfaces(input: &str) -> PartSolution {
    let cubes = parse_input(input);

    let unconnected_sides = cubes
        .iter()
        .flat_map(Cube::neighbors)
        .filter(|cube| !cubes.contains(cube))
        .count();

    PartSolution::USize(unconnected_sides)
}

fn lower_upper_bounds(cubes: &HashSet<Cube>) -> (Cube, Cube) {
    cubes.iter().fold(
        (
            Cube {
                x: isize::MAX,
                y: isize::MAX,
                z: isize::MAX,
            },
            Cube {
                x: isize::MIN,
                y: isize::MIN,
                z: isize::MIN,
            },
        ),
        |(mut lower, mut upper), cube| {
            lower.x = lower.x.min(cube.x);
            lower.y = lower.y.min(cube.y);
            lower.z = lower.z.min(cube.z);
            upper.x = upper.x.max(cube.x);
            upper.y = upper.y.max(cube.y);
            upper.z = upper.z.max(cube.z);

            (lower, upper)
        },
    )
}

fn exposed(cubes: &HashSet<Cube>) -> HashSet<Cube> {
    let (lower, upper) = lower_upper_bounds(cubes);

    let mut exposed = HashSet::new();

    let start = Cube { x: 0, y: 0, z: 0 };

    let mut stack = vec![start.clone()];
    let mut seen: HashSet<Cube> = HashSet::from_iter(stack.clone());

    while let Some(cube) = stack.pop() {
        for neighbor in cube.neighbors() {
            // since we're going from outside to inside
            let is_outside = !neighbor.in_bounds(&lower, &upper);

            // and stop at if the neighbor is one of our cube
            if cubes.contains(&neighbor) || is_outside {
                continue;
            }

            if seen.insert(neighbor.clone()) {
                stack.push(neighbor.clone());
                // we never insert any inside neighbors
                exposed.insert(neighbor);
            }
        }
    }

    exposed
}

fn count_outside_surfaces(input: &str) -> PartSolution {
    let cubes = parse_input(input);

    let exposed = exposed(&cubes);

    let outside_sides = cubes
        .iter()
        .flat_map(Cube::neighbors)
        .filter(|cube| exposed.contains(cube))
        .count();

    PartSolution::USize(outside_sides)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        count_surfaces(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        count_outside_surfaces(input)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2022::shared::solution::{read_file, read_file_part};
        use advent_of_code_2022::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(3662, (Solution {}).part_1(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_1() {
            assert_eq!(
                10,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 1))
            );
        }
        #[test]
        fn example_2() {
            assert_eq!(
                64,
                (Solution {}).part_1(&read_file_part("examples", &DAY, 2))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::solution::{read_file, read_file_part};
        use advent_of_code_2022::shared::Parts;

        use crate::{Solution, DAY};

        #[test]
        fn outcome() {
            assert_eq!(2060, (Solution {}).part_2(&read_file("inputs", &DAY)));
        }

        #[test]
        fn example_1() {
            assert_eq!(
                10,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 1))
            );
        }

        #[test]
        fn example_2() {
            assert_eq!(
                58,
                (Solution {}).part_2(&read_file_part("examples", &DAY, 2))
            );
        }
    }
}
