//! Advent of Code - Day 14 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_14;
use error::Result;

/// Advent of Code Day 14 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day14").about(DAY_14)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
