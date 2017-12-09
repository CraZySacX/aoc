//! Advent of Code - Day 17 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_17;
use error::Result;

/// Advent of Code Day 17 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day17").about(DAY_17)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
