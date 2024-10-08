use anyhow::Result;
use clap::Parser;
use claps::{Part1Variant as P1Var, Part2Variant as P2Var, SimpleF64Wrapper as F64, TheArgs};

mod claps;
mod pt1;
mod pt2;

fn main() -> Result<()> {
    match TheArgs::parse() {
        TheArgs::Part1(P1Var::Learn) => pt1::learn()?,
        TheArgs::Part1(P1Var::Predict { x: F64(x) }) => pt1::predict(x)?,
        TheArgs::Part2(P2Var::Learn) => pt2::learn()?,
        TheArgs::Part2(P2Var::Predict {
            x: F64(x),
            y: F64(y),
        }) => pt2::predict(y / x)?,
    }

    Ok(())
}
