//! Advent of Code 2017 Days
use error::Result;
use run::AoCDay;
use std::io::BufRead;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;

/// Find the solution.
pub fn find_solution<T: BufRead>(reader: T, day: &AoCDay, is_second_star: bool) -> Result<u32> {
    match *day {
        AoCDay::AOCD01 => day1::find_solution(reader, is_second_star),
        AoCDay::AOCD02 => day2::find_solution(reader, is_second_star),
        AoCDay::AOCD03 => day3::find_solution(reader, is_second_star),
        AoCDay::AOCD04 => day4::find_solution(reader, is_second_star),
        AoCDay::AOCD05 => day5::find_solution(reader, is_second_star),
        AoCDay::AOCD06 => day6::find_solution(reader, is_second_star),
        AoCDay::AOCD07 => day7::find_solution(reader, is_second_star),
        AoCDay::AOCD08 => day8::find_solution(reader, is_second_star),
        AoCDay::AOCD09 => day9::find_solution(reader, is_second_star),
        AoCDay::AOCD10 => day10::find_solution(reader, is_second_star),
        AoCDay::AOCD11 => day11::find_solution(reader, is_second_star),
        AoCDay::AOCD12 => day12::find_solution(reader, is_second_star),
        AoCDay::AOCD13 => day13::find_solution(reader, is_second_star),
        AoCDay::AOCD14 => day14::find_solution(reader, is_second_star),
        AoCDay::AOCD15 => day15::find_solution(reader, is_second_star),
        AoCDay::AOCD16 => day16::find_solution(reader, is_second_star),
        AoCDay::AOCD17 => day17::find_solution(reader, is_second_star),
        AoCDay::AOCD18 => day18::find_solution(reader, is_second_star),
        AoCDay::AOCD19 => day19::find_solution(reader, is_second_star),
        AoCDay::AOCD20 => day20::find_solution(reader, is_second_star),
        AoCDay::AOCD21 => day21::find_solution(reader, is_second_star),
        AoCDay::AOCD22 => day22::find_solution(reader, is_second_star),
        AoCDay::AOCD23 => day23::find_solution(reader, is_second_star),
        AoCDay::AOCD24 => day24::find_solution(reader, is_second_star),
    }
}
