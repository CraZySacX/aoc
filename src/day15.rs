//! Advent of Code - Day 15 Solution
use clap::{App, SubCommand};
use constants::DAY_15;

/// Advent of Code Day 15 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day15").about(DAY_15)
}
