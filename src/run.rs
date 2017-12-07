// Copyright (c) 2017 aoc developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` runtime
use clap::{App, Arg, SubCommand};
use {day1, day2};
use error::Result;
use std::fs::File;
use std::io::{self, BufReader, Write};

/// CLI Runtime
pub fn run() -> Result<i32> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Run Advent of Code 2017 daily problems")
        .subcommand(
            SubCommand::with_name("day1")
                .about("Run the 'Inverse Captcha' solution (AoC 2017 - Day 1)")
                .arg(
                    Arg::with_name("lookahead")
                        .short("l")
                        .help("Change the lookeahead from the default of 1, to (input length) / 2"),
                )
                .arg(Arg::with_name("value").required(true)),
        )
        .subcommand(
            SubCommand::with_name("day2")
                .about("Run the 'Checksum' solution (AoC 2017 - Day 2)")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .takes_value(true)
                        .required(true)
                        .default_value("cs_aoc2"),
                )
                .arg(Arg::with_name("algorithm").short("a")),
        )
        .subcommand(
            SubCommand::with_name("day3")
                .about("Run the 'Spiral Memory' solution (AoC 2017 - Day 3)")
                .arg(Arg::with_name("value").required(true))
        )
        .get_matches();

    // Get the subcommand's ArgMatches instance
    if let Some(day1_matches) = matches.subcommand_matches("day1") {
        let value = day1_matches
            .value_of("value")
            .ok_or("This should never happen due to clap validation!")?;

        writeln!(
            io::stdout(),
            "{}",
            day1::val(value, day1_matches.is_present("lookahead"))
        )?;
    } else if let Some(day2_matches) = matches.subcommand_matches("day2") {
        let filename = day2_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        let checksum = day2::parse_and_checksum(reader, day2_matches.is_present("algorithm"))?;
        writeln!(io::stdout(), "Checksum: {}", checksum)?;
    } else if let Some(day3_matches) = matches.subcommand_matches("day3") {
        let value = day3_matches.value_of("value").ok_or("This should never happen due to clap validation!")?;
        writeln!(io::stdout(), "Steps: {}", 0)?;
    } else {
        writeln!(io::stdout(), "Please choose a day to run the solution for")?;
    }

    Ok(0)
}
