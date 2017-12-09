//! Advent of Code - Day 21 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_21;
use error::Result;

/// Advent of Code Day 21 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day21").about(DAY_21)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
