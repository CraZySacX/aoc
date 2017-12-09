//! Advent of Code - Day 15 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_15;
use error::Result;

/// Advent of Code Day 15 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day15").about(DAY_15)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
