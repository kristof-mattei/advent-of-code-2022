use std::{
    cell::{Cell, RefCell},
    error::Error,
};

use crate::shared::{Day, PartSolution};

#[derive(Hash, Eq, PartialEq)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::W => write!(f, "w"),
            Register::X => write!(f, "x"),
            Register::Y => write!(f, "y"),
            Register::Z => write!(f, "z"),
        }
    }
}
enum RegisterOrValue {
    Register(Register),
    Value(i64),
}

impl std::fmt::Display for RegisterOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterOrValue::Register(r) => write!(f, "{}", r),
            RegisterOrValue::Value(v) => write!(f, "{}", v),
        }
    }
}

enum Instruction {
    Input(Register),
    Add(Register, RegisterOrValue),
    Mul(Register, RegisterOrValue),
    Div(Register, RegisterOrValue),
    Mod(Register, RegisterOrValue),
    Eql(Register, RegisterOrValue),
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Input(r) => write!(f, "inp {}", r),
            Instruction::Add(r, v) => write!(f, "add {} {}", r, v),
            Instruction::Mul(r, v) => write!(f, "mul {} {}", r, v),
            Instruction::Div(r, v) => write!(f, "div {} {}", r, v),
            Instruction::Mod(r, v) => write!(f, "mod {} {}", r, v),
            Instruction::Eql(r, v) => write!(f, "eql {} {}", r, v),
        }
    }
}

struct Alu<'i> {
    w: Cell<i64>,
    x: Cell<i64>,
    y: Cell<i64>,
    z: Cell<i64>,
    instructions: &'i [Instruction],
    input: RefCell<Vec<u32>>,
}

impl<'i> Alu<'i> {
    fn new(instructions: &'i [Instruction], input: Vec<u32>) -> Self {
        Self {
            w: 0.into(),
            x: 0.into(),
            y: 0.into(),
            z: 0.into(),
            instructions,
            input: input.into(),
        }
    }

    fn get_register(&self, register: &Register) -> i64 {
        match register {
            Register::W => self.w.get(),
            Register::X => self.x.get(),
            Register::Y => self.y.get(),
            Register::Z => self.z.get(),
        }
    }

    fn set_register(&self, register: &Register, value: i64) {
        match register {
            Register::W => self.w.set(value),
            Register::X => self.x.set(value),
            Register::Y => self.y.set(value),
            Register::Z => self.z.set(value),
        }
    }

    fn get_from_register_or_self(&self, registrer_or_value: &RegisterOrValue) -> i64 {
        match registrer_or_value {
            RegisterOrValue::Register(r) => self.get_register(r),
            RegisterOrValue::Value(v) => *v,
        }
    }

    fn process(&self) -> Result<i64, Box<dyn Error>> {
        for ins in self.instructions {
            // println!(
            //     "w: {}, x: {}, y: {}, z: {}",
            //     self.w.get(),
            //     self.x.get(),
            //     self.y.get(),
            //     self.z.get()
            // );
            // println!("Processing {:?}", ins);
            match ins {
                Instruction::Input(r) => {
                    let pop = self.input.borrow_mut().pop().unwrap();
                    self.set_register(r, i64::try_from(pop).unwrap());
                },
                Instruction::Add(a, b) => {
                    let a_val = self.get_register(a);
                    let b_val = self.get_from_register_or_self(b);
                    self.set_register(a, a_val + b_val);
                },
                Instruction::Mul(a, b) => {
                    let a_val = self.get_register(a);
                    let b_val = self.get_from_register_or_self(b);
                    self.set_register(a, a_val * b_val);
                },
                Instruction::Div(a, b) => {
                    let a_val = self.get_register(a);
                    let b_val = self.get_from_register_or_self(b);

                    let result = a_val
                        .checked_div_euclid(b_val)
                        .ok_or_else(|| "Division by 0".to_string())?;

                    self.set_register(a, result);
                },
                Instruction::Mod(a, b) => {
                    let a_val = self.get_register(a);
                    let b_val = self.get_from_register_or_self(b);

                    let result = a_val
                        .checked_rem_euclid(b_val)
                        .ok_or_else(|| "Division by 0".to_string())?;

                    self.set_register(a, result);
                },
                Instruction::Eql(a, b) => {
                    let a_val = self.get_register(a);
                    let b_val = self.get_from_register_or_self(b);
                    self.set_register(a, i64::from(a_val == b_val));
                },
            }
        }

        Ok(self.z.get())
    }
}

fn parse_lines(input: &[&str]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input {
        let split = line.split(' ').collect::<Vec<_>>();

        let part2 = split.get(1).unwrap();

        let param1 = match *part2 {
            "w" => Register::W,
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            _ => unreachable!(),
        };

        let param2 = if let Some(part3) = split.get(2) {
            let p2 = match *part3 {
                "w" => RegisterOrValue::Register(Register::W),
                "x" => RegisterOrValue::Register(Register::X),
                "y" => RegisterOrValue::Register(Register::Y),
                "z" => RegisterOrValue::Register(Register::Z),
                x => RegisterOrValue::Value(x.parse::<i64>().unwrap()),
            };
            Some(p2)
        } else {
            None
        };

        let part1 = split.first().unwrap();

        instructions.push(match *part1 {
            "inp" => Instruction::Input(param1),
            "add" => Instruction::Add(param1, param2.unwrap()),
            "mul" => Instruction::Mul(param1, param2.unwrap()),
            "div" => Instruction::Div(param1, param2.unwrap()),
            "mod" => Instruction::Mod(param1, param2.unwrap()),
            "eql" => Instruction::Eql(param1, param2.unwrap()),
            _ => unreachable!(),
        });
    }

    instructions
}

fn number_to_vec(mut input: u64) -> Vec<u32> {
    let mut vec = Vec::new();

    while input != 0 {
        let v = (input % 10) as u32;

        vec.push(v);

        input /= 10;
    }

    vec
}

fn find_maximum_version_number(instructions: &[Instruction]) -> u64 {
    let mut v: u64 = 99_999_999_999_999;

    loop {
        let input = number_to_vec(v);

        v -= 1;

        if input.contains(&0) {
            continue;
        }

        let alu = Alu::new(instructions, input);

        if let Ok(n) = alu.process() {
            if n == 0 {
                println!("{} is a valid number", v);
                break;
            }
        }

        println!(
            "{:?}: {}, {}, {}, {}",
            v + 1,
            alu.w.get(),
            alu.x.get(),
            alu.y.get(),
            alu.z.get()
        );
    }

    v
}

pub struct Solution {}

impl Day for Solution {
    fn part_1(&self) -> PartSolution {
        let lines: Vec<&str> = include_str!("input.txt").lines().collect();

        let instructions = parse_lines(&lines);

        let result = find_maximum_version_number(&instructions);

        PartSolution::U64(result)
    }

    fn part_2(&self) -> PartSolution {
        let _lines: Vec<&str> = include_str!("input.txt").lines().collect();

        PartSolution::None
    }
}

#[cfg(test)]
mod test {

    fn get_example() -> Vec<&'static str> {
        include_str!("example.txt").lines().collect()
    }

    mod part_1 {
        use crate::day_24::{number_to_vec, parse_lines, Alu};

        use super::get_example;

        #[test]
        fn outcome() {
            // assert_eq!((Solution {}).part_1(), PartSolution::U32(329));
        }

        #[test]
        fn test_number_to_vec() {
            assert_eq!(vec![4, 3, 2, 1], number_to_vec(1234));
        }

        #[test]
        fn example() {
            let example_lines = get_example();

            let instructions = parse_lines(&example_lines);

            let alu = Alu::new(&instructions, vec![12, 4]);

            let result = alu.process();

            assert_eq!(1, result.unwrap());
        }
    }
}
