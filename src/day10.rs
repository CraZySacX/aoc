//! Advent of Code - Day 10 Solution
use clap::{App, SubCommand};
use constants::DAY_10;

/// Advent of Code Day 10 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day10").about(DAY_10)
}
