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
use std::io::{self, Write};

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

    let result = if let Some(day1_matches) = matches.subcommand_matches("day01") {
        day1::find_solution(day1_matches)?
    } else if let Some(day2_matches) = matches.subcommand_matches("day02") {
        day2::find_solution(day2_matches)?
    } else if let Some(day3_matches) = matches.subcommand_matches("day03") {
        day3::find_solution(day3_matches)?
    } else if let Some(day4_matches) = matches.subcommand_matches("day04") {
        day4::find_solution(day4_matches)?
    } else if let Some(day5_matches) = matches.subcommand_matches("day05") {
        day5::find_solution(day5_matches)?
    } else if let Some(day6_matches) = matches.subcommand_matches("day06") {
        day6::find_solution(day6_matches)?
    } else if let Some(day7_matches) = matches.subcommand_matches("day07") {
        day7::find_solution(day7_matches)?
    } else if let Some(day8_matches) = matches.subcommand_matches("day08") {
        day8::find_solution(day8_matches)?
    } else if let Some(day9_matches) = matches.subcommand_matches("day09") {
        day9::find_solution(day9_matches)?
    } else {
        return Err("Unable to determine the day you wish to run".into());
    };

    writeln!(io::stdout(), "{}", result)?;
    Ok(0)
}
