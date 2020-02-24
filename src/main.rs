#[macro_use]
extern crate lazy_static;
use rand::Rng;
use regex::Regex;
use std::{fmt::Display, str::FromStr};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
struct Opts {
    /// Optional aggregate function to apply to the collected rolls of a die.
    ///
    /// One of 'sum', 'avg', 'max', 'min'
    #[structopt(long)]
    aggregate: Option<Aggregate>,
    dice_coll: Vec<Dice>,
}

#[derive(Clone, Debug)]
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
            invalid => Err(ParseAggregateError::InvalidFormat(format!(
                "invalid input: {}",
                invalid
            )))?,
        })
    }
}

#[derive(Clone, Debug)]
enum ParseAggregateError {
    InvalidFormat(String),
}

impl ToString for ParseAggregateError {
    fn to_string(&self) -> String {
        format!("")
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Dice {
    count: u32,
    sides: u32,
}

impl Dice {
    fn new(count: u32, sides: u32) -> Self {
        Self { count, sides }
    }

    fn roll(&self) -> Vec<u32> {
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

fn main() {
    let opts = Opts::from_args();

    for d in opts.dice_coll {
        let rolls = d.roll();
        println!(
            "{} {}",
            d,
            match opts.aggregate {
                None => format!("{:?}", rolls),
                Some(Aggregate::Sum) => format!("{}", rolls.iter().sum::<u32>()),
                Some(Aggregate::Avg) => format!("{}", rolls.iter().sum::<u32>() / d.count),
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

impl FromStr for Dice {
    type Err = ParseDieError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref DIE_REGEX: Regex = Regex::new(r#"^(?P<count>\d*)d(?P<sides>\d+)$"#).unwrap();
        };
        let captures = DIE_REGEX
            .captures(s)
            .ok_or_else(|| ParseDieError::InvalidFormat("regex failed to capture".into()))?;

        let count = match captures.name("count") {
            Some(count) => {
                let s = count.as_str();
                if s.is_empty() {
                    1
                } else {
                    count.as_str().parse().map_err(|_| {
                        ParseDieError::InvalidFormat(format!("invalid count {:?}", count))
                    })?
                }
            }
            None => 1,
        };

        if count < 1 {
            return Err(ParseDieError::InvalidFormat(
                "count must be at least 1".into(),
            ));
        }

        let sides = captures
            .name("sides")
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .ok_or_else(|| ParseDieError::InvalidFormat("missing sides".into()))?;

        Ok(Dice::new(count, sides))
    }
}

#[derive(Clone, Debug, PartialEq)]
enum ParseDieError {
    InvalidFormat(String),
}

impl ToString for ParseDieError {
    fn to_string(&self) -> String {
        match self {
            ParseDieError::InvalidFormat(s) => format!("parsing die failed with message: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let expected = Dice { count: 6, sides: 5 };

        assert_eq!(expected, Dice::from_str("6d5").unwrap());
    }
}
