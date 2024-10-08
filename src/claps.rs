use clap::{Parser, Subcommand};
use std::str::FromStr;

#[derive(Parser)]
pub enum TheArgs {
    #[command(subcommand)]
    Part1(Part1Variant),
    #[command(subcommand)]
    Part2(Part2Variant),
}

#[derive(Subcommand, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Part1Variant {
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

#[derive(Subcommand, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Part2Variant {
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SimpleF64Wrapper(pub f64);

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
    fn assert_receiver_is_total_eq(&self) {
        assert!(self.0.is_normal())
    }
}
