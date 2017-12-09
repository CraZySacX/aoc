//! Advent of Code - Day 24 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_24;
use error::Result;

/// Advent of Code Day 24 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day24").about(DAY_24)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
