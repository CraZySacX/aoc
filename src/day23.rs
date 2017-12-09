//! Advent of Code - Day 23 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_23;
use error::Result;

/// Advent of Code Day 23 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day23").about(DAY_23)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
