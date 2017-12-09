//! Advent of Code - Day 19 Solution
use clap::{App, SubCommand};
use constants::DAY_19;

/// Advent of Code Day 19 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day19").about(DAY_19)
}
