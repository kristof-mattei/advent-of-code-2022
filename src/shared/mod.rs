pub trait Day {
    fn part_1(&self) -> PartSolution;
    fn part_2(&self) -> PartSolution;
}

#[derive(PartialEq, Eq, Debug)]
pub enum PartSolution {
    I32(i32),
    U32(u32),
    U64(u64),
    USize(usize),
    Vec(Vec<String>),
    #[allow(dead_code)]
    None,
}

impl From<i32> for PartSolution {
    fn from(v: i32) -> Self {
        PartSolution::I32(v)
    }
}

impl From<u32> for PartSolution {
    fn from(v: u32) -> Self {
        PartSolution::U32(v)
    }
}

impl From<u64> for PartSolution {
    fn from(v: u64) -> Self {
        PartSolution::U64(v)
    }
}

impl From<usize> for PartSolution {
    fn from(v: usize) -> Self {
        PartSolution::USize(v)
    }
}

impl From<Vec<String>> for PartSolution {
    fn from(v: Vec<String>) -> Self {
        PartSolution::Vec(v)
    }
}

impl std::fmt::Display for PartSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match &self {
            PartSolution::I32(x) => x.to_string(),
            PartSolution::U32(x) => x.to_string(),
            PartSolution::U64(x) => x.to_string(),
            PartSolution::USize(x) => x.to_string(),
            PartSolution::Vec(x) => format!("\n{}", x.join("\n")),
            PartSolution::None => "None".to_owned(),
        };

        write!(f, "{}", string)
    }
}
