//! Constants

use anyhow::{Error, Result, anyhow};

/// Advent of Code Year
pub enum AoCYear {
    /// Advent of Code 2015
    AOC2015,
    /// Advent of Code 2016
    AOC2016,
    /// Advent of Code 2017
    AOC2017,
    /// Advent of Code 2018
    AOC2018,
}

impl<'a> From<&'a AoCYear> for &'a str {
    fn from(year: &AoCYear) -> Self {
        match *year {
            AoCYear::AOC2015 => "2015",
            AoCYear::AOC2016 => "2016",
            AoCYear::AOC2017 => "2017",
            AoCYear::AOC2018 => "2018",
        }
    }
}

impl TryFrom<&str> for AoCYear {
    type Error = Error;
    fn try_from(year: &str) -> Result<Self> {
        match year {
            "2015" => Ok(AoCYear::AOC2015),
            "2016" => Ok(AoCYear::AOC2016),
            "2017" => Ok(AoCYear::AOC2017),
            "2018" => Ok(AoCYear::AOC2018),
            _ => Err(anyhow!("Unable to convert to year!")),
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
    /// Day 25
    AOCD25,
}

impl<'a> From<&'a AoCDay> for &'a str {
    fn from(year: &AoCDay) -> Self {
        match *year {
            AoCDay::AOCD01 => DAY_1,
            AoCDay::AOCD02 => DAY_2,
            AoCDay::AOCD03 => DAY_3,
            AoCDay::AOCD04 => DAY_4,
            AoCDay::AOCD05 => DAY_5,
            AoCDay::AOCD06 => DAY_6,
            AoCDay::AOCD07 => DAY_7,
            AoCDay::AOCD08 => DAY_8,
            AoCDay::AOCD09 => DAY_9,
            AoCDay::AOCD10 => DAY_10,
            AoCDay::AOCD11 => DAY_11,
            AoCDay::AOCD12 => DAY_12,
            AoCDay::AOCD13 => DAY_13,
            AoCDay::AOCD14 => DAY_14,
            AoCDay::AOCD15 => DAY_15,
            AoCDay::AOCD16 => DAY_16,
            AoCDay::AOCD17 => DAY_17,
            AoCDay::AOCD18 => DAY_18,
            AoCDay::AOCD19 => DAY_19,
            AoCDay::AOCD20 => DAY_20,
            AoCDay::AOCD21 => DAY_21,
            AoCDay::AOCD22 => DAY_22,
            AoCDay::AOCD23 => DAY_23,
            AoCDay::AOCD24 => DAY_24,
            AoCDay::AOCD25 => DAY_25,
        }
    }
}

/// Day 1
pub const DAY_1: &str = "day01";
/// Day 2
pub const DAY_2: &str = "day02";
/// Day 3
pub const DAY_3: &str = "day03";
/// Day 4
pub const DAY_4: &str = "day04";
/// Day 5
pub const DAY_5: &str = "day05";
/// Day 6
pub const DAY_6: &str = "day06";
/// Day 7
pub const DAY_7: &str = "day07";
/// Day 8
pub const DAY_8: &str = "day08";
/// Day 9
pub const DAY_9: &str = "day09";
/// Day 10
pub const DAY_10: &str = "day10";
/// Day 11
pub const DAY_11: &str = "day11";
/// Day 12
pub const DAY_12: &str = "day12";
/// Day 13
pub const DAY_13: &str = "day13";
/// Day 14
pub const DAY_14: &str = "day14";
/// Day 15
pub const DAY_15: &str = "day15";
/// Day 16
pub const DAY_16: &str = "day16";
/// Day 17
pub const DAY_17: &str = "day17";
/// Day 18
pub const DAY_18: &str = "day18";
/// Day 19
pub const DAY_19: &str = "day19";
/// Day 20
pub const DAY_20: &str = "day20";
/// Day 21
pub const DAY_21: &str = "day21";
/// Day 22
pub const DAY_22: &str = "day22";
/// Day 23
pub const DAY_23: &str = "day23";
/// Day 24
pub const DAY_24: &str = "day24";
/// Day 25
pub const DAY_25: &str = "day25";
