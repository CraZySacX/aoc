//! Advent of Code - Day 20 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_20;
use error::Result;

/// Advent of Code Day 20 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day20").about(DAY_20)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
