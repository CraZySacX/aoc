//! Advent of Code - Day 17 Solution
use clap::{App, Arg, ArgMatches, SubCommand};
use constants::DAY_17;
use error::Result;
use run::AoCYear;

/// Advent of Code Day 17 `SubCommand`
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("day17")
        .about(DAY_17)
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
                .default_value("data/day17/data_file"),
        )
}

/// Find the solution.
pub fn find_solution(_matches: &ArgMatches, _year: &AoCYear) -> Result<u32> {
    Err("Not Implemented!".into())
}
