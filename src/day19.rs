//! Advent of Code - Day 19 Solution
use clap::{App, ArgMatches, SubCommand};
use constants::DAY_19;
use error::Result;

/// Advent of Code Day 19 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day19").about(DAY_19)
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches) -> Result<u32> {
    Err("Not Implemented!".into())
}
