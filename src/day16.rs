//! Advent of Code - Day 16 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_16;
use error::Result;

/// Advent of Code Day 16 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day16").about(DAY_16)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
