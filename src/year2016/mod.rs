//! Advent of Code 2016 Days
use error::Result;
use run::AoCDay;
use std::io::BufRead;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
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
pub mod day25;

/// Find the solution.
pub fn find_solution<T: BufRead>(reader: T, day: &AoCDay, is_second_star: bool) -> Result<u32> {
    match *day {
        AoCDay::AOCD01 => day01::find_solution(reader, is_second_star),
        AoCDay::AOCD02 => day02::find_solution(reader, is_second_star),
        AoCDay::AOCD03 => day03::find_solution(reader, is_second_star),
        AoCDay::AOCD04 => day04::find_solution(reader, is_second_star),
        AoCDay::AOCD05 => day05::find_solution(reader, is_second_star),
        AoCDay::AOCD06 => day06::find_solution(reader, is_second_star),
        AoCDay::AOCD07 => day07::find_solution(reader, is_second_star),
        AoCDay::AOCD08 => day08::find_solution(reader, is_second_star),
        AoCDay::AOCD09 => day09::find_solution(reader, is_second_star),
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
        AoCDay::AOCD25 => day25::find_solution(reader, is_second_star),
    }
}
