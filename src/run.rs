// Copyright (c) 2017 aoc developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` runtime
use clap::{App, Arg, ArgMatches, SubCommand};
use constants::{self, AoCDay, AoCYear};
use error::Result;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;
use std::time::Instant;
use utils::{self, Prefix};
use year2015;
use year2016;
use year2017;
use year2018;

/// Advent of Code `SubCommand`
fn subcommand<'a, 'b>(day: &AoCDay) -> App<'a, 'b> {
    SubCommand::with_name(day.into())
        .about(constants::get_day_about(day))
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .default_value("data_file"),
        )
        .arg(
            Arg::with_name("second")
                .short("s")
                .long("second")
                .help("Run the alrgorithm to calculate the value for the 2nd star"),
        )
}

/// Find the solution.
pub fn find_solution(matches: &ArgMatches, year: &AoCYear, day: &AoCDay) -> Result<u32> {
    let year_str: &str = year.into();
    let day_str: &str = day.into();
    let mut filepath = PathBuf::from("data");
    filepath.push(year_str);
    filepath.push(day_str);
    filepath.push(matches.value_of("file").ok_or("Invalid filename!")?);

    let reader = BufReader::new(File::open(filepath)?);
    let is_second_star = matches.is_present("second");

    match *year {
        AoCYear::AOC2018 => Ok(year2018::find_solution(reader, day, is_second_star)?),
        AoCYear::AOC2017 => Ok(year2017::find_solution(reader, day, is_second_star)?),
        AoCYear::AOC2016 => Ok(year2016::find_solution(reader, day, is_second_star)?),
        AoCYear::AOC2015 => Ok(year2015::find_solution(reader, day, is_second_star)?),
    }
}

/// CLI Runtime
pub fn run() -> Result<i32> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Run Advent of Code daily problems")
        .usage("\u{1f31f}   solution: aoc <day>\n    \u{1f31f}\u{1f31f} solution: aoc <day> -s")
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .default_value("2018")
                .required(true)
                .help("Specify the year you wish to work with"),
        )
        .arg(
            Arg::with_name("time")
                .short("t")
                .long("time")
                .help("Generate benchmark time, in s, ms, us, or ns")
                .takes_value(true)
                .possible_values(&["ns", "us", "ms", "s"]),
        )
        .subcommand(subcommand(&AoCDay::AOCD01))
        .subcommand(subcommand(&AoCDay::AOCD02))
        .subcommand(subcommand(&AoCDay::AOCD03))
        .subcommand(subcommand(&AoCDay::AOCD04))
        .subcommand(subcommand(&AoCDay::AOCD05))
        .subcommand(subcommand(&AoCDay::AOCD06))
        .subcommand(subcommand(&AoCDay::AOCD07))
        .subcommand(subcommand(&AoCDay::AOCD08))
        .subcommand(subcommand(&AoCDay::AOCD09))
        .subcommand(subcommand(&AoCDay::AOCD10))
        .subcommand(subcommand(&AoCDay::AOCD11))
        .subcommand(subcommand(&AoCDay::AOCD12))
        .subcommand(subcommand(&AoCDay::AOCD13))
        .subcommand(subcommand(&AoCDay::AOCD14))
        .subcommand(subcommand(&AoCDay::AOCD15))
        .subcommand(subcommand(&AoCDay::AOCD16))
        .subcommand(subcommand(&AoCDay::AOCD17))
        .subcommand(subcommand(&AoCDay::AOCD18))
        .subcommand(subcommand(&AoCDay::AOCD19))
        .subcommand(subcommand(&AoCDay::AOCD20))
        .subcommand(subcommand(&AoCDay::AOCD21))
        .subcommand(subcommand(&AoCDay::AOCD22))
        .subcommand(subcommand(&AoCDay::AOCD23))
        .subcommand(subcommand(&AoCDay::AOCD24))
        .subcommand(subcommand(&AoCDay::AOCD25))
        .get_matches();

    let year: AoCYear = TryFrom::try_from(matches.value_of("year").ok_or("Invalid year!")?)?;

    let match_tuple = match matches.subcommand() {
        (constants::DAY_1, Some(matches)) => (matches, AoCDay::AOCD01),
        (constants::DAY_2, Some(matches)) => (matches, AoCDay::AOCD02),
        (constants::DAY_3, Some(matches)) => (matches, AoCDay::AOCD03),
        (constants::DAY_4, Some(matches)) => (matches, AoCDay::AOCD04),
        (constants::DAY_5, Some(matches)) => (matches, AoCDay::AOCD05),
        (constants::DAY_6, Some(matches)) => (matches, AoCDay::AOCD06),
        (constants::DAY_7, Some(matches)) => (matches, AoCDay::AOCD07),
        (constants::DAY_8, Some(matches)) => (matches, AoCDay::AOCD08),
        (constants::DAY_9, Some(matches)) => (matches, AoCDay::AOCD09),
        (constants::DAY_10, Some(matches)) => (matches, AoCDay::AOCD10),
        (constants::DAY_11, Some(matches)) => (matches, AoCDay::AOCD11),
        (constants::DAY_12, Some(matches)) => (matches, AoCDay::AOCD12),
        (constants::DAY_13, Some(matches)) => (matches, AoCDay::AOCD13),
        (constants::DAY_14, Some(matches)) => (matches, AoCDay::AOCD14),
        (constants::DAY_15, Some(matches)) => (matches, AoCDay::AOCD15),
        (constants::DAY_16, Some(matches)) => (matches, AoCDay::AOCD16),
        (constants::DAY_17, Some(matches)) => (matches, AoCDay::AOCD17),
        (constants::DAY_18, Some(matches)) => (matches, AoCDay::AOCD18),
        (constants::DAY_19, Some(matches)) => (matches, AoCDay::AOCD19),
        (constants::DAY_20, Some(matches)) => (matches, AoCDay::AOCD20),
        (constants::DAY_21, Some(matches)) => (matches, AoCDay::AOCD21),
        (constants::DAY_22, Some(matches)) => (matches, AoCDay::AOCD22),
        (constants::DAY_23, Some(matches)) => (matches, AoCDay::AOCD23),
        (constants::DAY_24, Some(matches)) => (matches, AoCDay::AOCD24),
        (constants::DAY_25, Some(matches)) => (matches, AoCDay::AOCD25),
        _ => return Err("Unable to determine the day you wish to run".into()),
    };

    let now = Instant::now();
    writeln!(io::stdout(), "{}", find_solution(match_tuple.0, &year, &match_tuple.1)?)?;
    let duration = now.elapsed();

    if let Some(output_type) = matches.value_of("time") {
        let prefix: Prefix = TryFrom::try_from(output_type)?;

        let elapsed = match prefix {
            Prefix::Nanos => utils::as_ns(&duration)?,
            Prefix::Micros => utils::as_us(&duration)?,
            Prefix::Millis => utils::as_ms(&duration)?,
            Prefix::Seconds => utils::as_s(&duration)?,
        };

        writeln!(io::stdout(), "Elapsed: {elapsed}{output_type}")?;
    }
    Ok(0)
}
