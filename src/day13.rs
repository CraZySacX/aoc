//! Advent of Code - Day 13 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_13;
use error::Result;

/// Advent of Code Day 13 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day13").about(DAY_13)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
