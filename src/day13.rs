//! Advent of Code - Day 13 Solution
use clap::{App, SubCommand};
use constants::DAY_13;

/// Advent of Code Day 13 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day13").about(DAY_13)
}
