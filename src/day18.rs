//! Advent of Code - Day 18 Solution
use clap::{App, SubCommand};
use constants::DAY_18;

/// Advent of Code Day 18 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day18").about(DAY_18)
}
