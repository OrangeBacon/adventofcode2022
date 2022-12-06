use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
    ops::RangeInclusive,
};

use clap::builder::ValueParser;

#[derive(Clone, Copy)]
pub struct RangeParser {
    start: usize,
    end: usize,
}

/// Identical to [`std::range::RangeInclusive`] except implements display
#[derive(Clone, Copy, Debug)]
pub struct DisplayRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug)]
pub enum RangeParserError {
    IntParseError(ParseIntError),
    NotRange(String),
    NumberTooSmall(usize, usize),
    NumberTooLarge(usize, usize),
    RangeBackwards(String, usize, usize),
}

impl RangeParser {
    /// Create a clap value parser for ranges between two values.  The start and
    /// end should both be inclusive.
    pub fn new(start: usize, end: usize) -> ValueParser {
        ValueParser::new(RangeParser { start, end }.parser())
    }

    fn parser(self) -> impl Clone + Fn(&str) -> Result<DisplayRange, RangeParserError> {
        move |value| {
            let parse = |num: &str| {
                let num = num.parse::<usize>()?;
                if num < self.start {
                    Err(RangeParserError::NumberTooSmall(num, self.start))
                } else if num > self.end {
                    Err(RangeParserError::NumberTooLarge(num, self.end))
                } else {
                    Ok(num)
                }
            };
            let (start, end) = if let Ok(num) = value.parse() {
                parse(value)?;
                (num, num)
            } else if value == ".." {
                // full range
                (self.start, self.end)
            } else if value.starts_with("..=") {
                // ..=b
                let num = parse(&value[3..])?;
                (self.start, num)
            } else if value.starts_with("..") {
                // ..b
                let num = parse(&value[2..])?;
                (self.start, num + 1)
            } else if value.ends_with("..") {
                // a..
                let num = parse(&value[..value.len() - 2])?;
                (num, self.end)
            } else if let Some((a, b)) = value.split_once("..") {
                // a..b or a..=b
                // a will be a number, b will either be a number or `=number`
                let a = parse(a)?;

                let b = if b.starts_with('=') {
                    parse(&b[1..])?
                } else {
                    parse(b)? + 1
                };

                (a, b)
            } else {
                // invalid
                return Err(RangeParserError::NotRange(value.to_string()));
            };

            if start > end {
                return Err(RangeParserError::RangeBackwards(
                    value.to_string(),
                    start,
                    end,
                ));
            }

            Ok(DisplayRange { start, end })
        }
    }
}

impl Display for DisplayRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}..={}", self.start, self.end)
        }
    }
}

impl IntoIterator for DisplayRange {
    type Item = usize;

    type IntoIter = RangeInclusive<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

impl DisplayRange {
    /// Create a range of length 1
    pub fn one(value: usize) -> Self {
        Self {
            start: value,
            end: value,
        }
    }
}

impl From<ParseIntError> for RangeParserError {
    fn from(err: ParseIntError) -> Self {
        RangeParserError::IntParseError(err)
    }
}

impl Display for RangeParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RangeParserError::IntParseError(err) => {
                write!(f, "Unable to parse integer: {}", err)
            }
            RangeParserError::NotRange(val) => {
                write!(
                    f,
                    "Unable to parse `{}` as either a day number or day number range",
                    val
                )
            }
            RangeParserError::NumberTooLarge(num, max) => {
                write!(f, "Number too large, got `{num}`, maximum is `{max}`")
            }
            RangeParserError::NumberTooSmall(num, min) => {
                write!(f, "Number too small, got `{num}`, minimum is `{min}`")
            }
            RangeParserError::RangeBackwards(range, start, end) => {
                write!(f, "Range `{range}` implies that {start} <= {end}")
            }
        }
    }
}
impl Error for RangeParserError {}
