//! Advent of Code - Day 14 Solution
use clap::{App, SubCommand};
use constants::DAY_14;

/// Advent of Code Day 14 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day14").about(DAY_14)
}
