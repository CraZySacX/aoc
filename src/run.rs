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
    } else if let Some(day10_matches) = matches.subcommand_matches("day10") {
        day10::find_solution(day10_matches)?
    } else if let Some(day11_matches) = matches.subcommand_matches("day11") {
        day11::find_solution(day11_matches)?
    } else if let Some(day12_matches) = matches.subcommand_matches("day12") {
        day12::find_solution(day12_matches)?
    } else if let Some(day13_matches) = matches.subcommand_matches("day13") {
        day13::find_solution(day13_matches)?
    } else if let Some(day14_matches) = matches.subcommand_matches("day14") {
        day14::find_solution(day14_matches)?
    } else if let Some(day15_matches) = matches.subcommand_matches("day15") {
        day15::find_solution(day15_matches)?
    } else if let Some(day16_matches) = matches.subcommand_matches("day16") {
        day16::find_solution(day16_matches)?
    } else if let Some(day17_matches) = matches.subcommand_matches("day17") {
        day17::find_solution(day17_matches)?
    } else if let Some(day18_matches) = matches.subcommand_matches("day18") {
        day18::find_solution(day18_matches)?
    } else if let Some(day19_matches) = matches.subcommand_matches("day19") {
        day19::find_solution(day19_matches)?
    } else if let Some(day20_matches) = matches.subcommand_matches("day20") {
        day20::find_solution(day20_matches)?
    } else if let Some(day21_matches) = matches.subcommand_matches("day21") {
        day21::find_solution(day21_matches)?
    } else if let Some(day22_matches) = matches.subcommand_matches("day22") {
        day22::find_solution(day22_matches)?
    } else if let Some(day23_matches) = matches.subcommand_matches("day23") {
        day23::find_solution(day23_matches)?
    } else if let Some(day24_matches) = matches.subcommand_matches("day24") {
        day24::find_solution(day24_matches)?
    } else {
        return Err("Unable to determine the day you wish to run".into());
    };

    writeln!(io::stdout(), "{}", result)?;
    Ok(0)
}
