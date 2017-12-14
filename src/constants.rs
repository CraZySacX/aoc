//! Constants
use run::AoCDay;

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

/// Advent of Code Day 1 about string
pub const DAY_1_ABOUT: &str = "Inverse Captcha                        (Advent of Code 2017 - Day 01)";
/// Advent of Code Day 2 about string
pub const DAY_2_ABOUT: &str = "Corruption Checksum                    (Advent of Code 2017 - Day 02)";
/// Advent of Code Day 3 about string
pub const DAY_3_ABOUT: &str = "Spiral Memory                          (Advent of Code 2017 - Day 03)";
/// Advent of Code Day 4 about string
pub const DAY_4_ABOUT: &str = "High Entropy Passphrases               (Advent of Code 2017 - Day 04)";
/// Advent of Code Day 5 about string
pub const DAY_5_ABOUT: &str = "A Maze of Twisty Trampolines All Alike (Advent of Code 2017 - Day 05)";
/// Advent of Code Day 6 about string
pub const DAY_6_ABOUT: &str = "Memory Reallocation                    (Advent of Code 2017 - Day 06)";
/// Advent of Code Day 7 about string
pub const DAY_7_ABOUT: &str = "Recursive Circus                       (Advent of Code 2017 - Day 07)";
/// Advent of Code Day 8 about string
pub const DAY_8_ABOUT: &str = "I Heard You Like Registers             (Advent of Code 2017 - Day 08)";
/// Advent of Code Day 9 about string
pub const DAY_9_ABOUT: &str = "Stream Processing                      (Advent of Code 2017 - Day 09)";
/// Advent of Code Day 10 about string
pub const DAY_10_ABOUT: &str = "Knot Hash                              (Advent of Code 2017 - Day 10)";
/// Advent of Code Day 11 about string
pub const DAY_11_ABOUT: &str = "Hex Ed                                 (Advent of Code 2017 - Day 11)";
/// Advent of Code Day 12 about string
pub const DAY_12_ABOUT: &str = "Digital Plumber                        (Advent of Code 2017 - Day 12)";
/// Advent of Code Day 13 about string
pub const DAY_13_ABOUT: &str = "Packet Scanners                        (Advent of Code 2017 - Day 13)";
/// Advent of Code Day 14 about string
pub const DAY_14_ABOUT: &str = "                                       (Advent of Code 2017 - Day 14)";
/// Advent of Code Day 15 about string
pub const DAY_15_ABOUT: &str = "                                       (Advent of Code 2017 - Day 15)";
/// Advent of Code Day 16 about string
pub const DAY_16_ABOUT: &str = "                                       (Advent of Code 2017 - Day 16)";
/// Advent of Code Day 17 about string
pub const DAY_17_ABOUT: &str = "                                       (Advent of Code 2017 - Day 17)";
/// Advent of Code Day 18 about string
pub const DAY_18_ABOUT: &str = "                                       (Advent of Code 2017 - Day 18)";
/// Advent of Code Day 19 about string
pub const DAY_19_ABOUT: &str = "                                       (Advent of Code 2017 - Day 19)";
/// Advent of Code Day 20 about string
pub const DAY_20_ABOUT: &str = "                                       (Advent of Code 2017 - Day 20)";
/// Advent of Code Day 21 about string
pub const DAY_21_ABOUT: &str = "                                       (Advent of Code 2017 - Day 21)";
/// Advent of Code Day 22 about string
pub const DAY_22_ABOUT: &str = "                                       (Advent of Code 2017 - Day 22)";
/// Advent of Code Day 23 about string
pub const DAY_23_ABOUT: &str = "                                       (Advent of Code 2017 - Day 23)";
/// Advent of Code Day 24 about string
pub const DAY_24_ABOUT: &str = "                                       (Advent of Code 2017 - Day 24)";
/// Advent of Code Day 25 about string
pub const DAY_25_ABOUT: &str = "                                       (Advent of Code 2017 - Day 25)";

/// Get the constants tuple for a given day.
pub fn get_day_about<'a>(day: &AoCDay) -> &'a str {
    match *day {
        AoCDay::AOCD01 => DAY_1_ABOUT,
        AoCDay::AOCD02 => DAY_2_ABOUT,
        AoCDay::AOCD03 => DAY_3_ABOUT,
        AoCDay::AOCD04 => DAY_4_ABOUT,
        AoCDay::AOCD05 => DAY_5_ABOUT,
        AoCDay::AOCD06 => DAY_6_ABOUT,
        AoCDay::AOCD07 => DAY_7_ABOUT,
        AoCDay::AOCD08 => DAY_8_ABOUT,
        AoCDay::AOCD09 => DAY_9_ABOUT,
        AoCDay::AOCD10 => DAY_10_ABOUT,
        AoCDay::AOCD11 => DAY_11_ABOUT,
        AoCDay::AOCD12 => DAY_12_ABOUT,
        AoCDay::AOCD13 => DAY_13_ABOUT,
        AoCDay::AOCD14 => DAY_14_ABOUT,
        AoCDay::AOCD15 => DAY_15_ABOUT,
        AoCDay::AOCD16 => DAY_16_ABOUT,
        AoCDay::AOCD17 => DAY_17_ABOUT,
        AoCDay::AOCD18 => DAY_18_ABOUT,
        AoCDay::AOCD19 => DAY_19_ABOUT,
        AoCDay::AOCD20 => DAY_20_ABOUT,
        AoCDay::AOCD21 => DAY_21_ABOUT,
        AoCDay::AOCD22 => DAY_22_ABOUT,
        AoCDay::AOCD23 => DAY_23_ABOUT,
        AoCDay::AOCD24 => DAY_24_ABOUT,
        AoCDay::AOCD25 => DAY_25_ABOUT,
    }
}
