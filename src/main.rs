use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

mod pt1;
mod pt2;

#[derive(Subcommand, ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Part1Variant {
    Predict,
    Learn,
}

impl std::fmt::Display for Part1Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part1Variant::Predict => write!(f, "Predict"),
            Part1Variant::Learn => write!(f, "Learn"),
        }
    }
}

#[derive(Parser)]
enum TheArgs {
    #[command(subcommand)]
    Part1(Part1Variant),
    Part2,
}

fn main() -> Result<()> {
    match TheArgs::parse() {
        TheArgs::Part1(Part1Variant::Learn) => pt1::learn()?,
        TheArgs::Part1(Part1Variant::Predict) => pt1::predict()?,
        TheArgs::Part2 => pt2::main()?,
    }

    Ok(())
}
