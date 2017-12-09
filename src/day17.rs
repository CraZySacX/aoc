//! Advent of Code - Day 17 Solution
use clap::{App, SubCommand};
use constants::DAY_17;

/// Advent of Code Day 17 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day17").about(DAY_17)
}
