//! Advent of Code - Day 18 Solution
use clap::{App, Arg, ArgMatches, SubCommand};
use constants::DAY_18;
use error::Result;
use run::AoCYear;

/// Advent of Code Day 18 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day18")
        .about(DAY_18)
        .arg(
            Arg::with_name("second")
                .short("s")
                .long("second")
                .help("Run the alrgorithm to calculate the value for the 2nd star"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .default_value("data/day18/data_file"),
        )
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches, _year: &AoCYear) -> Result<u32> {
    Err("Not Implemented!".into())
}
