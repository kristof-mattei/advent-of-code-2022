use crate::shared::{Day, PartSolution};

// fn get_neighbors<T>(chiton_field: &[Vec<T>], coordinates: Coordinates) -> Vec<Coordinates> {
// }

#[derive(PartialEq, Eq, Clone, Copy)]
enum Pixel {
    Light,
    Dark,
}

impl std::fmt::Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "#"),
            Self::Dark => write!(f, "."),
        }
    }
}

struct Field {
    picture: Vec<Vec<Pixel>>,
    outer: Pixel,
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.picture {
            for col in row {
                write!(f, "{:?}", col)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn to_pixel(input: &str) -> Vec<Pixel> {
    input
        .chars()
        .map(|x| if x == '#' { Pixel::Light } else { Pixel::Dark })
        .collect()
}

fn parse_lookup(input: &[Pixel]) -> usize {
    assert_eq!(9, input.len());

    let binary = input
        .iter()
        .map(|p| match p {
            Pixel::Light => '1',
            Pixel::Dark => '0',
        })
        .collect::<String>();

    usize::from_str_radix(&binary, 2).unwrap()
}

fn get_lookup(field: &Field, row_index: usize, column_index: usize) -> Vec<Pixel> {
    let can_go_left = column_index > 0;
    let can_go_up = row_index > 0;

    let lookups = vec![
        // left up
        if can_go_left && can_go_up {
            field
                .picture
                .get(row_index - 1)
                .and_then(|r| r.get(column_index - 1))
        } else {
            None
        },
        if can_go_up {
            // up
            field
                .picture
                .get(row_index - 1)
                .and_then(|r| r.get(column_index))
        } else {
            None
        },
        // right up
        if can_go_up {
            field
                .picture
                .get(row_index - 1)
                .and_then(|r| r.get(column_index + 1))
        } else {
            None
        },
        // left
        if can_go_left && can_go_up {
            field
                .picture
                .get(row_index)
                .and_then(|r| r.get(column_index - 1))
        } else {
            None
        },
        // self
        field
            .picture
            .get(row_index)
            .and_then(|r| r.get(column_index)),
        // right
        field
            .picture
            .get(row_index)
            .and_then(|r| r.get(column_index + 1)),
        // left down
        // left up
        if can_go_left && can_go_up {
            field
                .picture
                .get(row_index + 1)
                .and_then(|r| r.get(column_index - 1))
        } else {
            None
        },
        // down
        field
            .picture
            .get(row_index + 1)
            .and_then(|r| r.get(column_index)),
        // right down
        field
            .picture
            .get(row_index + 1)
            .and_then(|r| r.get(column_index + 1)),
    ];

    lookups
        .iter()
        .map(|x| match *x {
            Some(p) => *p,
            None => field.outer,
        })
        .collect()
}

fn enhance(field: &Field, algorithm: &[Pixel]) -> Field {
    let outer = field.outer;

    println!("Before : \n{:?}", &field);

    let mut new_image = Vec::new();

    for (row_index, row) in field.picture.iter().enumerate() {
        let mut new_row = Vec::new();

        for (col_index, _pixel) in row.iter().enumerate() {
            let lookup = get_lookup(field, row_index, col_index);

            let translated = algorithm[parse_lookup(&lookup)];

            new_row.push(translated);
        }

        new_image.push(new_row);
    }

    let new_outer = match &outer {
        Pixel::Light => algorithm[0b1_1111_1111],
        Pixel::Dark => algorithm[0b0_0000_0000],
    };

    let field = Field {
        picture: new_image,
        outer: new_outer,
    };

    println!("After: \n{:?}", &field);

    field
}

fn parse_lines(lines: &[&str]) -> (Field, Vec<Pixel>) {
    // first line is the algoritm
    let algorithm = to_pixel(lines[0]);

    let mut picture = Vec::new();

    for line in lines.iter().skip(2) {
        picture.push(to_pixel(line));
    }

    (
        Field {
            picture,
            outer: Pixel::Dark,
        },
        algorithm,
    )
}

fn zoom_out(field: Field) -> Field {
    let columns = field.picture.get(0).map(Vec::len).unwrap();

    let add = 1;

    let mut outside = Vec::new();
    outside.resize(columns + 2 * add, field.outer);

    let mut new = vec![outside.clone(); add];

    for row in field.picture {
        let mut new_row = vec![field.outer; add];

        for col in row {
            new_row.push(col);
        }

        for _ in 0..add {
            new_row.push(field.outer);
        }

        new.push(new_row);
    }

    for _ in 0..add {
        new.push(outside.clone());
    }

    Field {
        picture: new,
        outer: field.outer,
    }
}

fn count_lit_pixels(field: &Field) -> u32 {
    let mut count: u32 = 0;

    for row in &field.picture {
        for col in row {
            if col == &Pixel::Light {
                count += 1;
            }
        }
    }

    count
}
pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let (field, algorithm) = parse_lines(&lines);
        // println!("{:?}", field);

        let field = enhance(&field, &algorithm);
        // println!("{:?}", field);

        let field = enhance(&field, &algorithm);
        println!("{:?}", field);
        let lit_pixels = count_lit_pixels(&field);

        PartSolution::U32(lit_pixels)
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
            day_20::{
                count_lit_pixels, enhance, get_lookup, parse_lines, parse_lookup, to_pixel, Pixel,
                Solution,
            },
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

            let (field, algorithm) = parse_lines(&example_lines);
            // println!("{:?}", field);

            let field = enhance(&field, &algorithm);
            // println!("{:?}", field);

            let field = enhance(&field, &algorithm);
            println!("{:?}", field);

            let lit_pixels = count_lit_pixels(&field);

            assert_eq!(35, lit_pixels);
        }

        #[test]
        fn test_to_pixel() {
            let pixels = to_pixel("#..#.");

            assert_eq!(
                vec![
                    Pixel::Light,
                    Pixel::Dark,
                    Pixel::Dark,
                    Pixel::Light,
                    Pixel::Dark
                ],
                pixels
            );
        }

        #[test]
        fn test_to_lookup() {
            let lookup = parse_lookup(&to_pixel("...#...#."));

            assert_eq!(34, lookup);
        }

        #[test]
        fn test_get_lookup() {
            let example_lines = get_example();

            let (field, algorithm) = parse_lines(&example_lines);

            // println!("{:?}", field);

            let lookup = get_lookup(&field, 2, 2);

            let parsed_lookup = parse_lookup(&lookup);

            let expected_algorithm_lookup = algorithm[parse_lookup(&to_pixel("...#...#."))];

            assert_eq!(expected_algorithm_lookup, algorithm[parsed_lookup]);
        }
    }

    mod part_2 {

        use crate::{
            day_20::Solution,
            shared::{Day, PartSolution},
        };

        #[test]
        fn outcome() {
            assert_eq!((Solution {}).part_2(), PartSolution::None);
        }
    }
}
