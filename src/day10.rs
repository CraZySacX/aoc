//! Advent of Code - Day 10 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_10;
use error::Result;

/// Advent of Code Day 10 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day10").about(DAY_10)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
