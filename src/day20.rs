//! Advent of Code - Day 20 Solution
use clap::{App, SubCommand};
use constants::DAY_20;

/// Advent of Code Day 20 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day20").about(DAY_20)
}
