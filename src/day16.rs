//! Advent of Code - Day 16 Solution
use clap::{App, SubCommand};
use constants::DAY_16;

/// Advent of Code Day 16 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day16").about(DAY_16)
}
