//! Advent of Code - Day 22 Solution
use clap::{App, SubCommand};
use constants::DAY_22;

/// Advent of Code Day 22 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day22").about(DAY_22)
}
