// Copyright (c) 2017 aoc developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` runtime
use clap::{App, Arg, SubCommand};
use {day1, day2, day3, day4, day5, day6, day7, day8};
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
                .about(
                    "Run the 'Inverse Captcha' solution                        (AoC 2017 - Day 1)",
                )
                .arg(
                    Arg::with_name("second")
                        .short("s")
                        .long("second")
                        .help("Run the alrgorithm to calculate the value for the 2nd star"),
                )
                .arg(Arg::with_name("value").required(true)),
        )
        .subcommand(
            SubCommand::with_name("day2")
                .about(
                    "Run the 'Corruption Checksum' solution                    (AoC 2017 - Day 2)",
                )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .takes_value(true)
                        .required(true)
                        .default_value("data/day2/cs_aoc2_actual"),
                )
                .arg(
                    Arg::with_name("second")
                        .short("s")
                        .long("second")
                        .help("Run the alrgorithm to calculate the value for the 2nd star"),
                ),
        )
        .subcommand(
            SubCommand::with_name("day3")
                .about(
                    "Run the 'Spiral Memory' solution                          (AoC 2017 - Day 3)",
                )
                .arg(
                    Arg::with_name("second")
                        .short("s")
                        .long("second")
                        .help("Run the alrgorithm to calculate the value for the 2nd star"),
                )
                .arg(Arg::with_name("value").required(true)),
        )
        .subcommand(
            SubCommand::with_name("day4")
                .about(
                    "Run the 'High Entropy Passphrases' solution               (AoC 2017 - Day 4)",
                )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .takes_value(true)
                        .required(true)
                        .default_value("data/day4/passphrase_list"),
                )
                .arg(
                    Arg::with_name("second")
                        .short("s")
                        .long("second")
                        .help("Run the alrgorithm to calculate the value for the 2nd star"),
                ),
        )
        .subcommand(
            SubCommand::with_name("day5")
                .about(
                    "Run the 'A Maze of Twisty Trampolines All Alike' solution (AoC 2017 - Day 5)",
                )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .takes_value(true)
                        .required(true)
                        .default_value("data/day5/jump_list"),
                )
                .arg(
                    Arg::with_name("second")
                        .short("s")
                        .long("second")
                        .help("Run the alrgorithm to calculate the value for the 2nd star"),
                ),
        )
        .subcommand(
            SubCommand::with_name("day6")
                .about(
                    "Run the 'Memory Reallocation' solution                    (AoC 2017 - Day 6)",
                )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .takes_value(true)
                        .required(true)
                        .default_value("data/day6/blocks_list"),
                )
                .arg(
                    Arg::with_name("second")
                        .short("s")
                        .long("second")
                        .help("Run the alrgorithm to calculate the value for the 2nd star"),
                ),
        )
        .subcommand(
            SubCommand::with_name("day7")
                .about(
                    "Run the 'Recursive Circus' solution                       (AoC 2017 - Day 7)",
                )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .takes_value(true)
                        .required(true)
                        .default_value("data/day7/node_list"),
                )
                .arg(
                    Arg::with_name("second")
                        .short("s")
                        .long("second")
                        .help("Run the alrgorithm to calculate the value for the 2nd star"),
                ),
        )
        .subcommand(
            SubCommand::with_name("day8")
                .about(
                    "Run the 'I Heard You Like Registers' solution             (AoC 2017 - Day 8)",
                )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .takes_value(true)
                        .required(true)
                        .default_value("data/day8/register_commands"),
                )
                .arg(
                    Arg::with_name("second")
                        .short("s")
                        .long("second")
                        .help("Run the alrgorithm to calculate the value for the 2nd star"),
                ),
        )
        .get_matches();

    let mut result: u32 = 0;

    if let Some(day1_matches) = matches.subcommand_matches("day1") {
        let value = day1_matches
            .value_of("value")
            .ok_or("This should never happen due to clap validation!")?;

        result = day1::val(value, day1_matches.is_present("second"))?;
    } else if let Some(day2_matches) = matches.subcommand_matches("day2") {
        let filename = day2_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day2::parse_and_checksum(reader, day2_matches.is_present("second"))?;
    } else if let Some(day3_matches) = matches.subcommand_matches("day3") {
        let value = day3_matches
            .value_of("value")
            .ok_or("This should never happen due to clap validation!")?;

        if day3_matches.is_present("second") {
            result = day3::next_biggest(value.parse::<u32>()?)?;
        } else {
            result = day3::calculate_steps(value.parse::<u32>()?)?;
        }
    } else if let Some(day4_matches) = matches.subcommand_matches("day4") {
        let filename = day4_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day4::count_valid_passphrases(reader, day4_matches.is_present("second"))?;
    } else if let Some(day5_matches) = matches.subcommand_matches("day5") {
        let filename = day5_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day5::jumps_until_exit(reader, day5_matches.is_present("second"))?;
    } else if let Some(day6_matches) = matches.subcommand_matches("day6") {
        let filename = day6_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day6::reallocations_until_match(reader, day6_matches.is_present("second"))?;
    } else if let Some(day7_matches) = matches.subcommand_matches("day7") {
        let filename = day7_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day7::build_tree(reader, day7_matches.is_present("second"))?;
    } else if let Some(day8_matches) = matches.subcommand_matches("day8") {
        let filename = day8_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day8::largest_register_value(reader, day8_matches.is_present("second"))?;
    } else {
        writeln!(io::stderr(), "Please choose a day to run the solution for")?;
    }

    writeln!(io::stdout(), "{}", result)?;
    Ok(0)
}
