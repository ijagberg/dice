#[macro_use]
extern crate lazy_static;
use rand::Rng;
use regex::Regex;
use std::{fmt::Display, num::ParseIntError, str::FromStr};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
struct Opts {
    /// Optional aggregate function to apply to the collected rolls of a die.
    ///
    /// One of 'sum', 'avg', 'max', 'min'
    #[structopt(short, long)]
    aggregate: Option<Aggregate>,

    /// Dice to roll. Eg. "d6", "5d10" etc
    dice: Vec<Dice>,
}

#[derive(Copy, Clone, Debug)]
enum Aggregate {
    Sum,
    Avg,
    Max,
    Min,
}

impl FromStr for Aggregate {
    type Err = ParseAggregateError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_uppercase().as_str() {
            "SUM" => Self::Sum,
            "AVG" => Self::Avg,
            "MAX" => Self::Max,
            "MIN" => Self::Min,
            _invalid => Err(ParseAggregateError::InvalidFunction)?,
        })
    }
}

#[derive(Clone, Debug)]
enum ParseAggregateError {
    InvalidFunction,
}

impl Display for ParseAggregateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ParseAggregateError::InvalidFunction => "invalid aggregate function",
        };
        write!(f, "{}", output)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dice {
    count: u32,
    sides: u32,
}

impl Dice {
    pub fn new(count: u32, sides: u32) -> Self {
        if count == 0 {
            panic!("count must be greater than 0");
        }
        if sides <= 1 {
            panic!("sides must be greater than 1");
        }
        Self { count, sides }
    }

    pub fn roll(&self) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        (0..self.count)
            .map(|_| rng.gen_range(1, self.sides + 1))
            .collect()
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.count, self.sides)
    }
}

impl FromStr for Dice {
    type Err = ParseDieError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref DIE_REGEX: Regex = Regex::new(r#"^(?P<count>\d*)d(?P<sides>\d+)$"#).unwrap();
        };
        let captures = DIE_REGEX
            .captures(s)
            .ok_or_else(|| ParseDieError::RegexFailedToCapture)?;

        let count = match captures.name("count") {
            Some(count) => {
                let s = count.as_str();
                if s.is_empty() {
                    1
                } else {
                    count
                        .as_str()
                        .parse()
                        .map_err(|e| ParseDieError::InvalidCount(e))?
                }
            }
            None => 1,
        };

        let sides = captures
            .name("sides")
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .ok_or_else(|| ParseDieError::MissingSides)?;

        Ok(Dice::new(count, sides))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseDieError {
    RegexFailedToCapture,
    InvalidCount(ParseIntError),
    MissingSides,
}

impl Display for ParseDieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ParseDieError::RegexFailedToCapture => {
                "regular expression could not capture contents of the input".to_string()
            }
            ParseDieError::InvalidCount(c) => format!("parsing failed with error: '{}'", c),
            ParseDieError::MissingSides => "could not match sides".to_string(),
        };
        write!(f, "{}", output)
    }
}

fn main() {
    let opts = Opts::from_args();

    if opts.dice.is_empty() {
        eprintln!("Provide some dice to roll")
    }

    for dice in opts.dice {
        let rolls = dice.roll();
        println!(
            "{} {}",
            dice,
            match opts.aggregate {
                None => rolls
                    .iter()
                    .map(|roll| roll.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
                Some(Aggregate::Sum) => format!("{}", rolls.iter().sum::<u32>()),
                Some(Aggregate::Avg) =>
                    format!("{}", rolls.iter().sum::<u32>() as f32 / dice.count as f32),
                Some(Aggregate::Max) => format!(
                    "{}",
                    rolls
                        .iter()
                        .max()
                        .expect("called aggregate max on empty iter")
                ),
                Some(Aggregate::Min) => format!(
                    "{}",
                    rolls
                        .iter()
                        .min()
                        .expect("called aggregate min on empty iter")
                ),
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let expected = Dice::new(6, 5);
        assert_eq!(expected, Dice::from_str("6d5").unwrap());

        let expected = Dice::new(1, 120);
        assert_eq!(expected, Dice::from_str("d120").unwrap());

        assert!(Dice::from_str("-5d20").is_err());
        assert!(Dice::from_str("5d-20").is_err());
        assert!(Dice::from_str("5d").is_err());
    }

    #[test]
    fn test_roll() {
        for count in 1..=100 {
            for sides in 2..=100 {
                let dice = Dice::new(count, sides);
                let rolls = dice.roll();
                for roll in rolls {
                    assert!(roll >= 1 && roll <= sides);
                }
            }
        }
    }
}
