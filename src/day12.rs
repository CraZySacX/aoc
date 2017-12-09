//! Advent of Code - Day 12 Solution
use clap::{App, SubCommand};
use constants::DAY_12;

/// Advent of Code Day 12 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day12").about(DAY_12)
}
