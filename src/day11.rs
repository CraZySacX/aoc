//! Advent of Code - Day 11 Solution
use clap::{App, SubCommand};
use constants::DAY_11;

/// Advent of Code Day 11 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day11").about(DAY_11)
}
