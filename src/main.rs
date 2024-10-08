use std::str::FromStr;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod pt1;
mod pt2;

#[derive(Copy, Clone, Debug, PartialEq)]
struct SimpleF64Wrapper(f64);

impl From<SimpleF64Wrapper> for f64 {
    fn from(value: SimpleF64Wrapper) -> Self {
        value.0
    }
}

impl From<f64> for SimpleF64Wrapper {
    fn from(value: f64) -> Self {
        SimpleF64Wrapper(value)
    }
}

impl FromStr for SimpleF64Wrapper {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        f64::from_str(s).map(Into::into)
    }
}

impl Eq for SimpleF64Wrapper {
    fn assert_receiver_is_total_eq(&self) {}
}

#[derive(Subcommand, Copy, Clone, Debug, PartialEq, Eq)]
enum Part2Variant {
    Predict {
        #[arg(value_name = "x")]
        x: SimpleF64Wrapper,
        #[arg(value_name = "y")]
        y: SimpleF64Wrapper,
    },
    Learn,
}

impl std::fmt::Display for Part2Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part2Variant::Predict { .. } => write!(f, "Predict"),
            Part2Variant::Learn => write!(f, "Learn"),
        }
    }
}

#[derive(Subcommand, Copy, Clone, Debug, PartialEq, Eq)]
enum Part1Variant {
    Predict {
        #[arg(value_name = "x")]
        x: SimpleF64Wrapper,
    },
    Learn,
}

impl std::fmt::Display for Part1Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part1Variant::Predict { .. } => write!(f, "Predict"),
            Part1Variant::Learn => write!(f, "Learn"),
        }
    }
}

#[derive(Parser)]
enum TheArgs {
    #[command(subcommand)]
    Part1(Part1Variant),
    #[command(subcommand)]
    Part2(Part2Variant),
}

fn main() -> Result<()> {
    use SimpleF64Wrapper as F64;

    match TheArgs::parse() {
        TheArgs::Part1(Part1Variant::Learn) => pt1::learn()?,
        TheArgs::Part1(Part1Variant::Predict { x: F64(x) }) => pt1::predict(x)?,
        TheArgs::Part2(Part2Variant::Learn) => pt2::learn()?,
        TheArgs::Part2(Part2Variant::Predict {
            x: F64(x),
            y: F64(y),
        }) => pt2::predict(y / x)?,
    }

    Ok(())
}
