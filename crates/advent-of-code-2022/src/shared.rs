use std::cmp::Ordering;

pub mod day;
pub mod grids;
pub mod solution;
pub mod tree;

pub trait Parts {
    fn part_1(&self, input: &str) -> PartSolution;
    fn part_2(&self, input: &str) -> PartSolution;
}

pub enum PartSolution {
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    ISize(isize),
    USize(usize),
    String(String),
    Vec(Vec<String>),
    Manual,
    None,
}

impl std::fmt::Debug for PartSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::I32(arg0) => write!(f, "{}i32", arg0),
            Self::U32(arg0) => write!(f, "{}u32", arg0),
            Self::I64(arg0) => write!(f, "{}i64", arg0),
            Self::U64(arg0) => write!(f, "{}u64", arg0),
            Self::ISize(arg0) => write!(f, "{}isize", arg0),
            Self::USize(arg0) => write!(f, "{}usize", arg0),
            Self::String(ref arg0) => write!(f, "\"{}\"", arg0),
            Self::Vec(ref arg0) => write!(f, "{:?}", arg0),
            Self::Manual => write!(f, "Manual"),
            Self::None => write!(f, "None"),
        }
    }
}

impl PartSolution {
    #[must_use]
    pub fn has_solution(&self) -> bool {
        !matches!(*self, PartSolution::None)
    }
}

impl PartialEq<PartSolution> for PartSolution {
    fn eq(&self, other: &PartSolution) -> bool {
        match *self {
            PartSolution::I32(ref i) => i == other,
            PartSolution::U32(ref i) => i == other,
            PartSolution::I64(ref i) => i == other,
            PartSolution::U64(ref i) => i == other,
            PartSolution::ISize(ref i) => i == other,
            PartSolution::USize(ref i) => i == other,
            PartSolution::String(ref i) => i == other,
            PartSolution::Vec(ref i) => i == other,
            PartSolution::None => matches!(other, &PartSolution::None),
            PartSolution::Manual => matches!(other, &PartSolution::Manual),
        }
    }
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

impl From<i64> for PartSolution {
    fn from(v: i64) -> Self {
        PartSolution::I64(v)
    }
}

impl From<u64> for PartSolution {
    fn from(v: u64) -> Self {
        PartSolution::U64(v)
    }
}

impl From<isize> for PartSolution {
    fn from(v: isize) -> Self {
        PartSolution::ISize(v)
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

impl From<String> for PartSolution {
    fn from(v: String) -> Self {
        PartSolution::String(v)
    }
}

impl From<&'_ str> for PartSolution {
    fn from(v: &'_ str) -> Self {
        PartSolution::String(v.into())
    }
}

impl From<Option<PartSolution>> for PartSolution {
    fn from(value: Option<PartSolution>) -> Self {
        match value {
            Some(v) => v,
            None => PartSolution::None,
        }
    }
}

impl std::fmt::Display for PartSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match *self {
            PartSolution::I32(other) => other.to_string(),
            PartSolution::U32(other) => other.to_string(),
            PartSolution::I64(other) => other.to_string(),
            PartSolution::U64(other) => other.to_string(),
            PartSolution::ISize(other) => other.to_string(),
            PartSolution::USize(other) => other.to_string(),
            PartSolution::String(ref other) => other.to_owned(),
            PartSolution::Vec(ref other) => format!("\n{}", other.join("\n")),
            PartSolution::Manual => "Manual".to_owned(),
            PartSolution::None => "None".to_owned(),
        };

        write!(f, "{}", string)
    }
}

impl std::cmp::PartialEq<PartSolution> for i32 {
    fn eq(&self, other: &PartSolution) -> bool {
        match *other {
            PartSolution::I32(other) => *self == other,
            PartSolution::U32(other) => i64::from(*self) == i64::from(other),
            PartSolution::I64(other) => i64::from(*self) == other,
            PartSolution::U64(other) => Ok(*self) == Self::try_from(other),
            PartSolution::ISize(other) => Ok(*self) == Self::try_from(other),
            PartSolution::USize(other) => Ok(*self) == Self::try_from(other),
            PartSolution::String(_)
            | PartSolution::Vec(_)
            | PartSolution::Manual
            | PartSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for u32 {
    fn eq(&self, other: &PartSolution) -> bool {
        match *other {
            PartSolution::I32(other) => i64::from(*self) == i64::from(other),
            PartSolution::U32(other) => *self == other,
            PartSolution::I64(other) => i64::from(*self) == other,
            PartSolution::U64(other) => u64::from(*self) == other,
            PartSolution::ISize(other) => Ok(*self) == Self::try_from(other),
            PartSolution::USize(other) => Ok(*self) == Self::try_from(other),
            PartSolution::String(_)
            | PartSolution::Vec(_)
            | PartSolution::Manual
            | PartSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for i64 {
    fn eq(&self, other: &PartSolution) -> bool {
        match *other {
            PartSolution::I32(other) => *self == Self::from(other),
            PartSolution::U32(other) => *self == Self::from(other),
            PartSolution::I64(other) => *self == other,
            PartSolution::U64(other) => Ok(*self) == Self::try_from(other),
            PartSolution::ISize(other) => Ok(*self) == Self::try_from(other),
            PartSolution::USize(other) => Ok(*self) == Self::try_from(other),
            PartSolution::String(_)
            | PartSolution::Vec(_)
            | PartSolution::Manual
            | PartSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for u64 {
    fn eq(&self, other: &PartSolution) -> bool {
        match *other {
            PartSolution::I32(other) => Ok(*self) == Self::try_from(other),
            PartSolution::U32(other) => *self == Self::from(other),
            PartSolution::I64(other) => Ok(*self) == Self::try_from(other),
            PartSolution::U64(other) => *self == other,
            PartSolution::ISize(other) => Ok(*self) == Self::try_from(other),
            PartSolution::USize(other) => Ok(*self) == Self::try_from(other),
            PartSolution::String(_)
            | PartSolution::Vec(_)
            | PartSolution::Manual
            | PartSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for isize {
    fn eq(&self, other: &PartSolution) -> bool {
        match *other {
            PartSolution::I32(other) => Ok(*self) == Self::try_from(other),
            PartSolution::U32(other) => Ok(*self) == Self::try_from(other),
            PartSolution::I64(other) => Ok(*self) == Self::try_from(other),
            PartSolution::U64(other) => Ok(*self) == Self::try_from(other),
            PartSolution::ISize(other) => *self == other,
            PartSolution::USize(other) => Ok(*self) == Self::try_from(other),
            PartSolution::String(_)
            | PartSolution::Vec(_)
            | PartSolution::Manual
            | PartSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for usize {
    fn eq(&self, other: &PartSolution) -> bool {
        match *other {
            PartSolution::I32(other) => Ok(*self) == Self::try_from(other),
            PartSolution::U32(other) => Ok(*self) == Self::try_from(other),
            PartSolution::I64(other) => Ok(*self) == Self::try_from(other),
            PartSolution::U64(other) => Ok(*self) == Self::try_from(other),
            PartSolution::ISize(other) => Ok(*self) == Self::try_from(other),
            PartSolution::USize(other) => *self == other,
            PartSolution::String(_)
            | PartSolution::Vec(_)
            | PartSolution::Manual
            | PartSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for String {
    fn eq(&self, other: &PartSolution) -> bool {
        match *other {
            PartSolution::String(ref s) => s == self,
            PartSolution::I32(_)
            | PartSolution::U32(_)
            | PartSolution::I64(_)
            | PartSolution::U64(_)
            | PartSolution::ISize(_)
            | PartSolution::USize(_)
            | PartSolution::Vec(_)
            | PartSolution::Manual
            | PartSolution::None => false,
        }
    }
}

impl std::cmp::PartialEq<PartSolution> for Vec<String> {
    fn eq(&self, other: &PartSolution) -> bool {
        matches!(*other, PartSolution::Vec(ref v) if {
            if v.len() != self.len() {
                return false;
            }

            for (l, r) in self.iter().zip(v) {
                if l != r {
                    return false;
                }
            }

            true
        })
    }
}

impl std::cmp::PartialOrd<PartSolution> for i32 {
    fn partial_cmp(&self, other: &PartSolution) -> Option<Ordering> {
        match *other {
            PartSolution::I32(ref other) => self.partial_cmp(other),
            PartSolution::U32(other) => i64::from(*self).partial_cmp(&i64::from(other)),
            PartSolution::I64(ref other) => i64::from(*self).partial_cmp(other),
            PartSolution::U64(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other doesn't fit into the smaller i32, meaning self is Less
                    Some(Ordering::Less)
                }
            },
            PartSolution::USize(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other doesn't fit into the smaller i32, meaning self is Less
                    Some(Ordering::Less)
                }
            },
            PartSolution::ISize(_)
            | PartSolution::String(_)
            | PartSolution::Vec(_)
            | PartSolution::Manual
            | PartSolution::None => None,
        }
    }
}

impl std::cmp::PartialOrd<PartSolution> for u32 {
    fn partial_cmp(&self, other: &PartSolution) -> Option<Ordering> {
        match *other {
            PartSolution::I32(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other is negative, so we are by definition Greater
                    Some(Ordering::Greater)
                }
            },
            PartSolution::U32(ref other) => self.partial_cmp(other),
            PartSolution::I64(ref other) => i64::from(*self).partial_cmp(other),
            PartSolution::U64(ref other) => u64::from(*self).partial_cmp(other),
            PartSolution::USize(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other doesn't fit into the smaller u32, meaning self is Less
                    Some(Ordering::Less)
                }
            },
            PartSolution::ISize(_)
            | PartSolution::String(_)
            | PartSolution::Vec(_)
            | PartSolution::Manual
            | PartSolution::None => None,
        }
    }
}

impl std::cmp::PartialOrd<PartSolution> for u64 {
    fn partial_cmp(&self, other: &PartSolution) -> Option<Ordering> {
        match *other {
            PartSolution::I32(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other is negative, so we are by definition Greater
                    Some(Ordering::Greater)
                }
            },
            PartSolution::U32(other) => self.partial_cmp(&u64::from(other)),
            PartSolution::I64(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other is negative, so we are by definition Greater
                    Some(Ordering::Greater)
                }
            },
            PartSolution::U64(ref other) => self.partial_cmp(other),
            PartSolution::USize(other) => {
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other doesn't fit into usize, meaning self is Less
                    Some(Ordering::Less)
                }
            },
            PartSolution::ISize(_)
            | PartSolution::String(_)
            | PartSolution::Vec(_)
            | PartSolution::Manual
            | PartSolution::None => None,
        }
    }
}

impl std::cmp::PartialOrd<PartSolution> for usize {
    fn partial_cmp(&self, other: &PartSolution) -> Option<Ordering> {
        match *other {
            PartSolution::I32(other) => {
                if other.is_negative() {
                    Some(Ordering::Greater)
                } else if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other is positive, but doesn't fit into usize, so we're Less
                    Some(Ordering::Less)
                }
            },
            PartSolution::U32(other) => {
                // if this fails, that means that usize is smaller than u32
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other doesn't fit into usize
                    Some(Ordering::Less)
                }
            },
            PartSolution::I64(other) => {
                if other.is_negative() {
                    Some(Ordering::Greater)
                } else if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other is positive, but doesn't fit into usize, so we're Less
                    Some(Ordering::Less)
                }
            },
            PartSolution::U64(other) => {
                // if this fails, that means that usize is smaller than u64
                if let Ok(other) = Self::try_from(other) {
                    self.partial_cmp(&other)
                } else {
                    // other doesn't fit into usize
                    Some(Ordering::Less)
                }
            },
            PartSolution::USize(other) => other.partial_cmp(self),
            PartSolution::ISize(_)
            | PartSolution::String(_)
            | PartSolution::Vec(_)
            | PartSolution::Manual
            | PartSolution::None => None,
        }
    }
}

impl std::cmp::PartialOrd<PartSolution> for String {
    fn partial_cmp(&self, other: &PartSolution) -> Option<Ordering> {
        match *other {
            PartSolution::String(ref s) => s.partial_cmp(self),
            PartSolution::I32(_)
            | PartSolution::U32(_)
            | PartSolution::I64(_)
            | PartSolution::U64(_)
            | PartSolution::ISize(_)
            | PartSolution::USize(_)
            | PartSolution::Vec(_)
            | PartSolution::Manual
            | PartSolution::None => None,
        }
    }
}

impl std::cmp::PartialOrd<PartSolution> for Vec<String> {
    fn partial_cmp(&self, other: &PartSolution) -> Option<Ordering> {
        match *other {
            PartSolution::Vec(ref v) => {
                if v.len() != self.len() {
                    return None;
                }

                for (l, r) in self.iter().zip(v) {
                    if l != r {
                        return None;
                    }
                }

                Some(Ordering::Equal)
            },
            PartSolution::I32(_)
            | PartSolution::U32(_)
            | PartSolution::I64(_)
            | PartSolution::U64(_)
            | PartSolution::ISize(_)
            | PartSolution::USize(_)
            | PartSolution::String(_)
            | PartSolution::Manual
            | PartSolution::None => None,
        }
    }
}
