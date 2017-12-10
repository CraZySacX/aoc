// Copyright (c) 2017 aoc developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` runtime
use clap::{App, Arg, ArgMatches, SubCommand};
use constants;
use error::Error;
use error::Result;
use seventeen::{day1, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day2, day20, day21, day22, day23, day24, day3, day4, day5, day6,
                day7, day8, day9};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufReader, Write};

/// Advent of Code Year
pub enum AoCYear {
    /// Advent of Code 2015
    AOC2015,
    /// Advent of Code 2016
    AOC2016,
    /// Advent of Code 2017
    AOC2017,
}

impl<'a> TryFrom<&'a str> for AoCYear {
    type Error = Error;
    fn try_from(year: &str) -> Result<Self> {
        match year {
            "2015" => Ok(AoCYear::AOC2015),
            "2016" => Ok(AoCYear::AOC2016),
            "2017" => Ok(AoCYear::AOC2017),
            _ => Err("Unable to convert to year!".into()),
        }
    }
}

/// Advent of Code Days
pub enum AoCDay {
    /// Day 1
    AOCD01,
    /// Day 2
    AOCD02,
    /// Day 3
    AOCD03,
    /// Day 4
    AOCD04,
    /// Day 5
    AOCD05,
    /// Day 6
    AOCD06,
    /// Day 7
    AOCD07,
    /// Day 8
    AOCD08,
    /// Day 9
    AOCD09,
    /// Day 10
    AOCD10,
    /// Day 11
    AOCD11,
    /// Day 12
    AOCD12,
    /// Day 13
    AOCD13,
    /// Day 14
    AOCD14,
    /// Day 15
    AOCD15,
    /// Day 16
    AOCD16,
    /// Day 17
    AOCD17,
    /// Day 18
    AOCD18,
    /// Day 19
    AOCD19,
    /// Day 20
    AOCD20,
    /// Day 21
    AOCD21,
    /// Day 22
    AOCD22,
    /// Day 23
    AOCD23,
    /// Day 24
    AOCD24,
}

/// Advent of Code `SubCommand`
fn subcommand<'a, 'b>(day: &AoCDay) -> App<'a, 'b> {
    let constant_tuple = constants::get_day_tuple(day);

    SubCommand::with_name(constant_tuple.0)
        .about(constant_tuple.1)
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true)
                .default_value(constant_tuple.2),
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
    let filename = matches.value_of("file").ok_or("Invalid filename!")?;
    let reader = BufReader::new(File::open(filename)?);
    let is_second_star = matches.is_present("second");

    match *day {
        AoCDay::AOCD01 => day1::find_solution(reader, year, is_second_star),
        AoCDay::AOCD02 => day2::find_solution(reader, year, is_second_star),
        AoCDay::AOCD03 => day3::find_solution(reader, year, is_second_star),
        AoCDay::AOCD04 => day4::find_solution(reader, year, is_second_star),
        AoCDay::AOCD05 => day5::find_solution(reader, year, is_second_star),
        AoCDay::AOCD06 => day6::find_solution(reader, year, is_second_star),
        AoCDay::AOCD07 => day7::find_solution(reader, year, is_second_star),
        AoCDay::AOCD08 => day8::find_solution(reader, year, is_second_star),
        AoCDay::AOCD09 => day9::find_solution(reader, year, is_second_star),
        AoCDay::AOCD10 => day10::find_solution(reader, year, is_second_star),
        AoCDay::AOCD11 => day11::find_solution(reader, year, is_second_star),
        AoCDay::AOCD12 => day12::find_solution(reader, year, is_second_star),
        AoCDay::AOCD13 => day13::find_solution(reader, year, is_second_star),
        AoCDay::AOCD14 => day14::find_solution(reader, year, is_second_star),
        AoCDay::AOCD15 => day15::find_solution(reader, year, is_second_star),
        AoCDay::AOCD16 => day16::find_solution(reader, year, is_second_star),
        AoCDay::AOCD17 => day17::find_solution(reader, year, is_second_star),
        AoCDay::AOCD18 => day18::find_solution(reader, year, is_second_star),
        AoCDay::AOCD19 => day19::find_solution(reader, year, is_second_star),
        AoCDay::AOCD20 => day20::find_solution(reader, year, is_second_star),
        AoCDay::AOCD21 => day21::find_solution(reader, year, is_second_star),
        AoCDay::AOCD22 => day22::find_solution(reader, year, is_second_star),
        AoCDay::AOCD23 => day23::find_solution(reader, year, is_second_star),
        AoCDay::AOCD24 => day24::find_solution(reader, year, is_second_star),
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
                .default_value("2017")
                .required(true)
                .help("Specify the year you wish to work with"),
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
        _ => return Err("Unable to determine the day you wish to run".into()),
    };

    writeln!(
        io::stdout(),
        "{}",
        find_solution(match_tuple.0, &year, &match_tuple.1)?
    )?;
    Ok(0)
}
