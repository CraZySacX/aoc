// Copyright (c) 2017 aoc developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` runtime
use {day1, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day2, day20, day21, day22, day23, day24, day3, day4, day5, day6, day7, day8,
     day9};
use clap::App;
use error::Result;
use std::fs::File;
use std::io::{self, BufReader, Write};

/// CLI Runtime
pub fn run() -> Result<i32> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Run Advent of Code 2017 daily problems")
        .usage("\u{1f31f}   solution: aoc <day>\n    \u{1f31f}\u{1f31f} solution: aoc <day> -s")
        .subcommand(day1::subcommand())
        .subcommand(day2::subcommand())
        .subcommand(day3::subcommand())
        .subcommand(day4::subcommand())
        .subcommand(day5::subcommand())
        .subcommand(day6::subcommand())
        .subcommand(day7::subcommand())
        .subcommand(day8::subcommand())
        .subcommand(day9::subcommand())
        .subcommand(day10::subcommand())
        .subcommand(day11::subcommand())
        .subcommand(day12::subcommand())
        .subcommand(day13::subcommand())
        .subcommand(day14::subcommand())
        .subcommand(day15::subcommand())
        .subcommand(day16::subcommand())
        .subcommand(day17::subcommand())
        .subcommand(day18::subcommand())
        .subcommand(day19::subcommand())
        .subcommand(day20::subcommand())
        .subcommand(day21::subcommand())
        .subcommand(day22::subcommand())
        .subcommand(day23::subcommand())
        .subcommand(day24::subcommand())
        .get_matches();

    let mut result: u32 = 0;

    if let Some(day1_matches) = matches.subcommand_matches("day01") {
        let value = day1_matches
            .value_of("value")
            .ok_or("This should never happen due to clap validation!")?;

        result = day1::val(value, day1_matches.is_present("second"))?;
    } else if let Some(day2_matches) = matches.subcommand_matches("day02") {
        let filename = day2_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day2::parse_and_checksum(reader, day2_matches.is_present("second"))?;
    } else if let Some(day3_matches) = matches.subcommand_matches("day03") {
        let value = day3_matches
            .value_of("value")
            .ok_or("This should never happen due to clap validation!")?;

        if day3_matches.is_present("second") {
            result = day3::next_biggest(value.parse::<u32>()?)?;
        } else {
            result = day3::calculate_steps(value.parse::<u32>()?)?;
        }
    } else if let Some(day4_matches) = matches.subcommand_matches("day04") {
        let filename = day4_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day4::count_valid_passphrases(reader, day4_matches.is_present("second"))?;
    } else if let Some(day5_matches) = matches.subcommand_matches("day05") {
        let filename = day5_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day5::jumps_until_exit(reader, day5_matches.is_present("second"))?;
    } else if let Some(day6_matches) = matches.subcommand_matches("day06") {
        let filename = day6_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day6::reallocations_until_match(reader, day6_matches.is_present("second"))?;
    } else if let Some(day7_matches) = matches.subcommand_matches("day07") {
        let filename = day7_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day7::build_tree(reader, day7_matches.is_present("second"))?;
    } else if let Some(day8_matches) = matches.subcommand_matches("day08") {
        let filename = day8_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day8::largest_register_value(reader, day8_matches.is_present("second"))?;
    } else if let Some(day9_matches) = matches.subcommand_matches("day09") {
        let filename = day9_matches.value_of("file").ok_or("Invalid filename!")?;
        let reader = BufReader::new(File::open(filename)?);
        result = day9::process_stream(reader, day9_matches.is_present("second"))?;
    } else {
        writeln!(io::stderr(), "Please choose a day to run the solution for")?;
    }

    writeln!(io::stdout(), "{}", result)?;
    Ok(0)
}
