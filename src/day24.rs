//! Advent of Code - Day 24 Solution
use clap::{App, SubCommand};
use constants::DAY_24;

/// Advent of Code Day 24 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day24").about(DAY_24)
}
