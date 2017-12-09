//! Advent of Code - Day 18 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_18;
use error::Result;

/// Advent of Code Day 18 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day18").about(DAY_18)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
