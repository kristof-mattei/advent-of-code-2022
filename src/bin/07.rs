use std::cell::RefCell;
use std::rc::Rc;

use advent_of_code_2022::shared::{PartSolution, Parts};

advent_of_code_2022::solution!(1_297_683, 5_756_764);

struct Directory {
    name: String,
    children: Vec<DirOrFile>,
}

impl std::fmt::Debug for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- {} (dir)", self.name)?;

        for child in &self.children {
            let s = format!("{:?}", child);

            for line in s.lines() {
                writeln!(f, "  {}", line)?;
            }
        }

        Ok(())
    }
}

enum DirOrFile {
    File { name: String, size: usize },
    Directory(Rc<RefCell<Directory>>),
}

impl std::fmt::Debug for DirOrFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            DirOrFile::Directory(d) => {
                let borrowed_dir = (*(*d)).borrow();
                write!(f, "{:?}", borrowed_dir)
            },
            DirOrFile::File { name, size } => {
                writeln!(f, "- {} (file, size={})", name, size)
            },
        }
    }
}

fn parse_lines(input: &str) -> Rc<RefCell<Directory>> {
    let lines = input.lines().collect::<Vec<_>>();

    let root = Rc::new(RefCell::new(Directory {
        name: "/".into(),
        children: vec![],
    }));

    let mut history: Vec<Rc<RefCell<Directory>>> = Vec::new();

    let mut current_line_index = 0;

    while let Some(line) = lines.get(current_line_index) {
        if line.starts_with("$ cd") {
            // parse as command
            match line[5..].trim() {
                "/" => {
                    history.push(Rc::clone(&root));
                },
                ".." => {
                    history.pop();
                },
                dir_name => {
                    let last = (*(*(history.last().unwrap()))).borrow();

                    let dir_reference = Rc::clone(
                        last.children
                            .iter()
                            .find_map(|c| match c {
                                DirOrFile::Directory(dir)
                                    if (*(*dir)).borrow().name == dir_name =>
                                {
                                    Some(dir)
                                },
                                _ => None,
                            })
                            .unwrap(),
                    );

                    drop(last);

                    history.push(dir_reference);
                },
            }

            current_line_index += 1;
            continue;
        }

        if line == &"$ ls" {
            // parse out the set of lines as directory listing of the history.last()
            let mut ls_index = current_line_index + 1;

            let mut contents = vec![];

            while let Some(&ls_line) = lines
                .get(ls_index)
                .filter(|&&ls_line| !ls_line.starts_with('$'))
            {
                let split = ls_line.split_whitespace().collect::<Vec<&str>>();

                let left = split[0];
                let right = split[1];

                if left == "dir" {
                    contents.push(DirOrFile::Directory(Rc::new(RefCell::new(Directory {
                        name: right.into(),
                        children: vec![],
                    }))));
                } else {
                    // file
                    contents.push(DirOrFile::File {
                        name: right.into(),
                        size: left.parse::<usize>().unwrap(),
                    });
                }

                ls_index += 1;

                let mut last = history.last().unwrap().borrow_mut();
                last.children.append(&mut contents);
            }

            current_line_index = ls_index;
            continue;
        }

        unreachable!()
    }

    root
}

fn dirs_smaller_than_100_000_r(sums: &mut Vec<usize>, dir: &Rc<RefCell<Directory>>) -> usize {
    let mut sum = 0;

    for child in &(*dir).borrow().children {
        match child {
            DirOrFile::File { name: _, size } => sum += *size,
            DirOrFile::Directory(d) => sum += dirs_smaller_than_100_000_r(sums, d),
        }
    }

    if sum <= 100_000 {
        sums.push(sum);
    }

    sum
}

fn dirs_smaller_than_100_000(root: &Rc<RefCell<Directory>>) -> usize {
    let mut sums = vec![];

    dirs_smaller_than_100_000_r(&mut sums, root);

    sums.into_iter().sum::<usize>()
}

fn sum_r(dir: &Rc<RefCell<Directory>>) -> usize {
    let mut sum = 0;

    for child in &(*dir).borrow().children {
        match child {
            DirOrFile::File { name: _, size } => sum += *size,
            DirOrFile::Directory(d) => sum += sum_r(d),
        }
    }

    sum
}
fn test(larger_than: &mut Vec<usize>, minimum_size: usize, dir: &Rc<RefCell<Directory>>) -> usize {
    let mut sum = 0;

    for child in &(*dir).borrow().children {
        match child {
            DirOrFile::File { name: _, size } => sum += *size,
            DirOrFile::Directory(d) => sum += test(larger_than, minimum_size, d),
        }
    }
    if sum >= minimum_size {
        larger_than.push(sum);
    }

    sum
}
fn find_smallest_dir_to_delete(root: &Rc<RefCell<Directory>>) -> usize {
    const TOTAL: usize = 70_000_000;
    const MINIMUM_FREE: usize = 30_000_000;

    let unused = TOTAL - sum_r(root);

    let to_free = MINIMUM_FREE - unused;

    let mut smaller_than_to_free = vec![];

    test(&mut smaller_than_to_free, to_free, root);

    smaller_than_to_free.sort_unstable();

    smaller_than_to_free[0]
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let tree = parse_lines(input);

        let sum_dirs_smaller_than_100_000 = dirs_smaller_than_100_000(&tree);

        sum_dirs_smaller_than_100_000.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let tree = parse_lines(input);

        let smallest_dir_to_delete_size = find_smallest_dir_to_delete(&tree);

        smallest_dir_to_delete_size.into()
    }
}

#[cfg(test)]
mod tests {

    mod part_1 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(1_297_683),
                (Solution {}).part_1(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(95_437),
                (Solution {}).part_1(&read_file("examples", &DAY))
            );
        }
    }

    mod part_2 {
        use advent_of_code_2022::shared::solution::read_file;
        use advent_of_code_2022::shared::{PartSolution, Parts};

        use crate::{DAY, Solution};

        #[test]
        fn outcome() {
            assert_eq!(
                PartSolution::USize(5_756_764),
                (Solution {}).part_2(&read_file("inputs", &DAY))
            );
        }

        #[test]
        fn example() {
            assert_eq!(
                PartSolution::USize(24_933_642),
                (Solution {}).part_2(&read_file("examples", &DAY))
            );
        }
    }
}
