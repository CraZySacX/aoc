// Copyright (c) 2017 aoc developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `aoc` runtime

use crate::{
    cli::{AoC2Subcommand, Args, Command},
    constants::{AoCDay, AoCYear},
    year2015, year2016, year2017, year2018,
};
use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufReader, Write},
    path::PathBuf,
};

/// Find the solution.
pub fn find_solution(matches: &AoC2Subcommand, year: &AoCYear, day: &AoCDay) -> Result<u32> {
    let year_str: &str = year.into();
    let day_str: &str = day.into();
    let mut filepath = PathBuf::from("data");
    filepath.push(year_str);
    filepath.push(day_str);
    filepath.push(matches.file().as_str());

    let reader = BufReader::new(File::open(filepath)?);
    let is_second_star = *matches.second();

    match *year {
        AoCYear::AOC2018 => Ok(year2018::find_solution(reader, day, is_second_star)?),
        AoCYear::AOC2017 => Ok(year2017::find_solution(reader, day, is_second_star)?),
        AoCYear::AOC2016 => Ok(year2016::find_solution(reader, day, is_second_star)?),
        AoCYear::AOC2015 => Ok(year2015::find_solution(reader, day, is_second_star)?),
    }
}

/// CLI Runtime
pub fn run() -> Result<i32> {
    // Parse the command line
    let matches = Args::try_parse()?;

    let year = AoCYear::try_from(&matches.year()[..])?;

    let match_tuple = match matches.command() {
        Command::Day01(command) => (command, AoCDay::AOCD01),
        Command::Day02(command) => (command, AoCDay::AOCD02),
        Command::Day03(command) => (command, AoCDay::AOCD03),
        Command::Day04(command) => (command, AoCDay::AOCD04),
        Command::Day05(command) => (command, AoCDay::AOCD05),
        Command::Day06(command) => (command, AoCDay::AOCD06),
        Command::Day07(command) => (command, AoCDay::AOCD07),
        Command::Day08(command) => (command, AoCDay::AOCD08),
        Command::Day09(command) => (command, AoCDay::AOCD09),
        Command::Day10(command) => (command, AoCDay::AOCD10),
        Command::Day11(command) => (command, AoCDay::AOCD11),
        Command::Day12(command) => (command, AoCDay::AOCD12),
        Command::Day13(command) => (command, AoCDay::AOCD13),
        Command::Day14(command) => (command, AoCDay::AOCD14),
        Command::Day15(command) => (command, AoCDay::AOCD15),
        Command::Day16(command) => (command, AoCDay::AOCD16),
        Command::Day17(command) => (command, AoCDay::AOCD17),
        Command::Day18(command) => (command, AoCDay::AOCD18),
        Command::Day19(command) => (command, AoCDay::AOCD19),
        Command::Day20(command) => (command, AoCDay::AOCD20),
        Command::Day21(command) => (command, AoCDay::AOCD21),
        Command::Day22(command) => (command, AoCDay::AOCD22),
        Command::Day23(command) => (command, AoCDay::AOCD23),
        Command::Day24(command) => (command, AoCDay::AOCD24),
        Command::Day25(command) => (command, AoCDay::AOCD25),
    };

    writeln!(io::stdout(), "{}", find_solution(match_tuple.0, &year, &match_tuple.1)?)?;
    Ok(0)
}
