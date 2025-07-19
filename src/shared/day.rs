use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

/// A valid day number of advent (i.e. an integer in range 1 to 25).
///
/// # Display
/// This value displays as a two digit number.
///
/// ```
/// # use advent_of_code_2022::shared::day::Day;
/// const DAY: Day = Day::try_new(8).unwrap();
/// assert_eq!(DAY.to_string(), "08")
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Day(u8);

impl Day {
    /// Creates a [`Day`] from the provided value if it's in the valid range,
    /// returns [`None`] otherwise.
    #[must_use]
    pub const fn try_new(day: u8) -> Option<Self> {
        if day == 0 || day > 25 {
            return None;
        }

        Some(Day(day))
    }

    // Not part of the public API
    #[must_use]
    #[doc(hidden)]
    pub const fn new_unchecked(day: u8) -> Self {
        Self(day)
    }

    /// Converts the [`Day`] into an [`u8`].
    #[must_use]
    pub fn into_inner(self) -> u8 {
        self.0
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl PartialEq<u8> for Day {
    fn eq(&self, other: &u8) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<u8> for Day {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

/* -------------------------------------------------------------------------- */

impl FromStr for Day {
    type Err = DayFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day = s.parse().map_err(|_| DayFromStrError {})?;
        Self::try_new(day).ok_or(DayFromStrError {})
    }
}

/// An error which can be returned when parsing a [`Day`].
#[expect(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct DayFromStrError {}

impl Error for DayFromStrError {}

impl Display for DayFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("expecting a day number between 1 and 25")
    }
}

/// Creates a [`Day`] value in a const context.
#[macro_export]
macro_rules! day {
    ($day:literal) => {{
        $crate::shared::day::Day::try_new($day).expect(concat!(
            "invalid day number `",
            $day,
            "`, expecting a value between 1 and 25"
        ))
    }};
}
