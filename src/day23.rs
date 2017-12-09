//! Advent of Code - Day 23 Solution
use clap::{App, SubCommand};
use constants::DAY_23;

/// Advent of Code Day 23 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day23").about(DAY_23)
}
