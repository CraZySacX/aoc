//! Advent of Code - Day 12 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_12;
use error::Result;

/// Advent of Code Day 12 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day12").about(DAY_12)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
