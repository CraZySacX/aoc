//! Advent of Code - Day 11 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_11;
use error::Result;

/// Advent of Code Day 11 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day11").about(DAY_11)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
