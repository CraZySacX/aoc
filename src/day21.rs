//! Advent of Code - Day 21 Solution
use clap::{App, SubCommand};
use constants::DAY_21;

/// Advent of Code Day 21 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day21").about(DAY_21)
}
