use std::hash::{Hash, Hasher};
use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::shared::{Day, PartSolution};

#[derive(Eq, Default)]
struct Cave {
    name: String,
    targets: RefCell<Caves>,
}

impl Cave {
    fn is_end(&self) -> bool {
        self.name == "end"
    }
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Cave {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

struct VisitableCave {
    cave: Rc<Cave>,
    visits: u32,
}

// impl VisitableCave {

//     fn visit(&self) {

//     }
// }

type Caves = HashSet<Rc<Cave>>;

#[allow(clippy::mutable_key_type)]
fn get_or_insert_cave(caves: &mut Caves, cave_name: String) -> Rc<Cave> {
    let cave = Rc::new(Cave {
        name: cave_name,
        ..Cave::default()
    });

    if let Some(found_cave) = caves.get(&cave) {
        found_cave.clone()
    } else {
        caves.insert(cave.clone());

        cave
    }
}

#[allow(clippy::mutable_key_type)]
fn add_path(caves: &mut Caves, from: String, to: String) {
    let from_cave = get_or_insert_cave(caves, from);
    let to_cave = get_or_insert_cave(caves, to);

    from_cave.targets.borrow_mut().insert(to_cave.clone());
    to_cave.targets.borrow_mut().insert(from_cave);
}

#[allow(clippy::mutable_key_type)]
fn build_cave_system(lines: &[String]) -> Caves {
    #[allow(clippy::mutable_key_type)]
    let mut caves: Caves = HashSet::default();

    for line in lines {
        let pieces: Vec<String> = line.split('-').map(Into::into).collect();

        let left = pieces.get(0).unwrap();
        let right = pieces.get(1).unwrap();

        add_path(&mut caves, left.clone(), right.clone());
    }

    caves
}

fn navigate_caves(cave: &Rc<Cave>, mut visited: Vec<String>) -> Vec<Vec<String>> {
    let mut solutions = Vec::new();
    visited.push(cave.name.clone());

    if cave.is_end() {
        println!("The end, we visited {:?} to get here.", visited);

        solutions.push(visited);
    } else {
        for target_cave in cave.targets.borrow().iter() {
            if target_cave.name.to_lowercase() != *target_cave.name
                || !visited.contains(&target_cave.name)
            {
                let visited_new: Vec<String> = visited.clone();

                println!("Visiting {} -> {}", cave.name, target_cave.name);
                for solution in navigate_caves(target_cave, visited_new) {
                    solutions.push(solution);
                }
            }
        }
    }

    solutions
}

#[allow(clippy::mutable_key_type)]
fn calculate_all_paths(cave_system: &Caves) -> usize {
    let start = cave_system.iter().find(|c| c.name == "start").unwrap();

    let solutions = navigate_caves(start, Vec::new());

    println!("{:?}", solutions);

    solutions.len()
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        #[allow(clippy::mutable_key_type)]
        let cave_system = build_cave_system(&lines);

        let paths: usize = calculate_all_paths(&cave_system);

        PartSolution::USize(paths)
    }

    fn part_2(&self) -> PartSolution {
        let _lines: Vec<String> = include_str!("input.txt").lines().map(Into::into).collect();

        PartSolution::None
    }
}

#[cfg(test)]
mod test {
    fn get_example() -> Vec<String> {
        include_str!("example.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    fn get_example_slightly_larger() -> Vec<String> {
        include_str!("example_slightly_larger.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    fn get_example_even_larger() -> Vec<String> {
        include_str!("example_even_larger.txt")
            .lines()
            .map(Into::into)
            .collect()
    }

    mod part_1 {

        use crate::{
            day_12::{build_cave_system, calculate_all_paths, Solution},
            shared::{Day, PartSolution},
        };

        use super::get_example;
        use super::get_example_even_larger;
        use super::get_example_slightly_larger;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_1(), PartSolution::USize(4495));
        }

        #[test]
        fn example() {
            let lines = get_example();

            #[allow(clippy::mutable_key_type)]
            let cave_system = build_cave_system(&lines);

            let paths: usize = calculate_all_paths(&cave_system);

            assert_eq!(paths, 10);
        }

        #[test]
        fn example_slightly_larger() {
            let lines = get_example_slightly_larger();

            #[allow(clippy::mutable_key_type)]
            let cave_system = build_cave_system(&lines);

            let paths: usize = calculate_all_paths(&cave_system);

            assert_eq!(paths, 19);
        }

        #[test]
        fn example_even_larger() {
            let lines = get_example_even_larger();

            #[allow(clippy::mutable_key_type)]
            let cave_system = build_cave_system(&lines);

            let paths: usize = calculate_all_paths(&cave_system);

            assert_eq!(paths, 226);
        }
    }

    mod part_2 {

        use crate::{
            day_12::Solution,
            shared::{Day, PartSolution},
        };

        use super::get_example;

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::None);
        }
        #[test]
        fn example() {
            let lines = get_example();

            assert!(!lines.is_empty());
        }
    }
}
